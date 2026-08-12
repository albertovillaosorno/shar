# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - The repository behavior implemented by this source file.
# - Must-Not:
#   - Bypass the contracts or authority boundaries of its owning package.
# - Allows:
#   - Inputs: values admitted by the file's public or internal interface.
#   - Outputs: deterministic values or effects declared by that interface.
#   - Side effects: only those explicitly owned by the implementation.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another file owns the exact same responsibility.
# - Summary:
#   - Small RFC 8439 ChaCha20-Poly1305 reference implementation.
# - Description:
#   - Implements the responsibility summarized by this module.
# - Usage:
#   - Used through the owning package, executable, or document boundary.
# - Defaults:
#   - Invalid inputs or broken invariants fail closed.
#

"""
Small RFC 8439 ChaCha20-Poly1305 reference implementation.

This implementation exists so authoring tests and the future generated Rust
materializer can share exact byte-level vectors without adding a package/runtime
dependency. It follows the RFC 8439 AEAD construction and deliberately exposes
only the combined authenticated-encryption operation needed by source-bound
literal payloads.

The Python Poly1305 arithmetic uses arbitrary-precision integers and is not a
constant-time implementation. It is suitable for deterministic local authoring
and verification, not as the eventual distributable runtime implementation.
"""

from __future__ import annotations

from dataclasses import dataclass
import hmac

_KEY_BYTES = 32
_NONCE_BYTES = 12
_TAG_BYTES = 16
_BLOCK_BYTES = 64
_WORD_MASK = 0xFFFFFFFF
_COUNTER_MAX = 0xFFFFFFFF
_POLY_MODULUS = (1 << 130) - 5
_POLY_TAG_MODULUS = 1 << 128
_POLY_R_MASK = 0x0FFFFFFC0FFFFFFC0FFFFFFC0FFFFFFF
_ONE = 1
_ZERO = 0


class PayloadCryptoError(ValueError):
    """Raised when authenticated payload inputs or tags are invalid."""


def _require_bytes(value: object, context: str) -> bytes:
    if type(value) is not bytes:
        message = f"{context} must use exact bytes"
        raise PayloadCryptoError(message)
    return value


@dataclass(frozen=True, slots=True)
class AuthenticatedPayload:
    """ChaCha20-Poly1305 ciphertext and detached 128-bit authentication tag."""

    ciphertext: bytes
    tag: bytes

    def __post_init__(self) -> None:
        """Require the RFC 8439 tag width.

        Raises:
            PayloadCryptoError: The authentication tag has the wrong width.

        """
        _ = _require_bytes(self.ciphertext, "ChaCha20-Poly1305 ciphertext")
        tag = _require_bytes(self.tag, "ChaCha20-Poly1305 tag")
        if len(tag) != _TAG_BYTES:
            message = "ChaCha20-Poly1305 tag must be 16 bytes"
            raise PayloadCryptoError(message)


def _require_key_nonce(key: object, nonce: object) -> None:
    key_bytes = _require_bytes(key, "ChaCha20-Poly1305 key")
    nonce_bytes = _require_bytes(nonce, "ChaCha20-Poly1305 nonce")
    if len(key_bytes) != _KEY_BYTES:
        message = "ChaCha20-Poly1305 key must be 32 bytes"
        raise PayloadCryptoError(message)
    if len(nonce_bytes) != _NONCE_BYTES:
        message = "ChaCha20-Poly1305 nonce must be 12 bytes"
        raise PayloadCryptoError(message)


def _rotate_left(value: int, amount: int) -> int:
    value &= _WORD_MASK
    return ((value << amount) | (value >> (32 - amount))) & _WORD_MASK


def _quarter_round(
    state: list[int], indexes: tuple[int, int, int, int]
) -> None:
    a, b, c, d = indexes
    state[a] = (state[a] + state[b]) & _WORD_MASK
    state[d] ^= state[a]
    state[d] = _rotate_left(state[d], 16)
    state[c] = (state[c] + state[d]) & _WORD_MASK
    state[b] ^= state[c]
    state[b] = _rotate_left(state[b], 12)
    state[a] = (state[a] + state[b]) & _WORD_MASK
    state[d] ^= state[a]
    state[d] = _rotate_left(state[d], 8)
    state[c] = (state[c] + state[d]) & _WORD_MASK
    state[b] ^= state[c]
    state[b] = _rotate_left(state[b], 7)


def _words(data: bytes) -> tuple[int, ...]:
    return tuple(
        int.from_bytes(data[offset : offset + 4], byteorder="little")
        for offset in range(_ZERO, len(data), 4)
    )


def _initial_state(key: bytes, nonce: bytes, counter: int) -> tuple[int, ...]:
    constants = _words(b"expand 32-byte k")
    return (
        *constants,
        *_words(key),
        counter,
        *_words(nonce),
    )


def _chacha20_block(key: bytes, nonce: bytes, counter: int) -> bytes:
    if counter < _ZERO or counter > _COUNTER_MAX:
        message = "ChaCha20 block counter exceeds 32-bit domain"
        raise PayloadCryptoError(message)
    initial = _initial_state(key, nonce, counter)
    state = list(initial)
    for _ in range(10):
        _quarter_round(state, (0, 4, 8, 12))
        _quarter_round(state, (1, 5, 9, 13))
        _quarter_round(state, (2, 6, 10, 14))
        _quarter_round(state, (3, 7, 11, 15))
        _quarter_round(state, (0, 5, 10, 15))
        _quarter_round(state, (1, 6, 11, 12))
        _quarter_round(state, (2, 7, 8, 13))
        _quarter_round(state, (3, 4, 9, 14))
    return b"".join(
        ((word + original) & _WORD_MASK).to_bytes(4, byteorder="little")
        for word, original in zip(state, initial, strict=True)
    )


def _block_count(length: int) -> int:
    return (length + _BLOCK_BYTES - _ONE) // _BLOCK_BYTES


def _chacha20_xor(
    key: bytes,
    nonce: bytes,
    data: bytes,
    *,
    initial_counter: int,
) -> bytes:
    blocks = _block_count(len(data))
    if blocks and initial_counter + blocks - _ONE > _COUNTER_MAX:
        message = "ChaCha20 payload exceeds the RFC 8439 counter space"
        raise PayloadCryptoError(message)
    output = bytearray(len(data))
    for block_index in range(blocks):
        offset = block_index * _BLOCK_BYTES
        block = data[offset : offset + _BLOCK_BYTES]
        stream = _chacha20_block(key, nonce, initial_counter + block_index)
        output[offset : offset + len(block)] = bytes(
            left ^ right
            for left, right in zip(block, stream[: len(block)], strict=True)
        )
    return bytes(output)


def _poly1305_mac(message: bytes, one_time_key: bytes) -> bytes:
    if len(one_time_key) != _KEY_BYTES:
        error_message = "Poly1305 one-time key must be 32 bytes"
        raise PayloadCryptoError(error_message)
    r = int.from_bytes(one_time_key[:16], byteorder="little") & _POLY_R_MASK
    s = int.from_bytes(one_time_key[16:], byteorder="little")
    accumulator = _ZERO
    for offset in range(_ZERO, len(message), 16):
        block = message[offset : offset + 16]
        value = int.from_bytes(block + b"\x01", byteorder="little")
        accumulator = ((accumulator + value) * r) % _POLY_MODULUS
    tag = (accumulator + s) % _POLY_TAG_MODULUS
    return tag.to_bytes(_TAG_BYTES, byteorder="little")


def _pad16(data: bytes) -> bytes:
    remainder = len(data) % 16
    return b"" if remainder == _ZERO else bytes(16 - remainder)


def _mac_data(aad: bytes, ciphertext: bytes) -> bytes:
    maximum = (1 << 64) - _ONE
    if len(aad) > maximum or len(ciphertext) > maximum:
        message = "ChaCha20-Poly1305 length exceeds 64-bit AEAD encoding"
        raise PayloadCryptoError(message)
    return b"".join((
        aad,
        _pad16(aad),
        ciphertext,
        _pad16(ciphertext),
        len(aad).to_bytes(8, byteorder="little"),
        len(ciphertext).to_bytes(8, byteorder="little"),
    ))


def chacha20_poly1305_encrypt(
    key: bytes,
    nonce: bytes,
    plaintext: bytes,
    *,
    aad: bytes = b"",
) -> AuthenticatedPayload:
    """Encrypt and authenticate one RFC 8439 AEAD message.

    Returns:
        Detached ciphertext and 16-byte Poly1305 tag.

    """
    _require_key_nonce(key, nonce)
    _ = _require_bytes(plaintext, "ChaCha20-Poly1305 plaintext")
    _ = _require_bytes(aad, "ChaCha20-Poly1305 associated data")
    one_time_key = _chacha20_block(key, nonce, _ZERO)[:_KEY_BYTES]
    ciphertext = _chacha20_xor(
        key,
        nonce,
        plaintext,
        initial_counter=_ONE,
    )
    tag = _poly1305_mac(_mac_data(aad, ciphertext), one_time_key)
    return AuthenticatedPayload(ciphertext=ciphertext, tag=tag)


def chacha20_poly1305_decrypt(
    key: bytes,
    nonce: bytes,
    payload: AuthenticatedPayload,
    *,
    aad: bytes = b"",
) -> bytes:
    """Authenticate before decrypting one RFC 8439 AEAD message.

    Returns:
        Authenticated plaintext bytes.

    Raises:
        PayloadCryptoError: Inputs are malformed or authentication fails.

    """
    _require_key_nonce(key, nonce)
    if type(payload) is not AuthenticatedPayload:
        message = (
            "ChaCha20-Poly1305 payload must use the exact authenticated type"
        )
        raise PayloadCryptoError(message)
    _ = _require_bytes(payload.ciphertext, "ChaCha20-Poly1305 ciphertext")
    _ = _require_bytes(payload.tag, "ChaCha20-Poly1305 tag")
    _ = _require_bytes(aad, "ChaCha20-Poly1305 associated data")
    one_time_key = _chacha20_block(key, nonce, _ZERO)[:_KEY_BYTES]
    expected = _poly1305_mac(
        _mac_data(aad, payload.ciphertext),
        one_time_key,
    )
    if not hmac.compare_digest(expected, payload.tag):
        message = "ChaCha20-Poly1305 authentication failed"
        raise PayloadCryptoError(message)
    return _chacha20_xor(
        key,
        nonce,
        payload.ciphertext,
        initial_counter=_ONE,
    )
