# Cartoon Network LP, LLLP v. CSC Holdings, Inc

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: Official Second Circuit opinion text verified from an archived court
  PDF; the former live court URL no longer resolves.
- Court: United States Court of Appeals for the Second Circuit.
- Authority: 536 F.3d 121 (2d Cir. 2008).
- Docket: 07-1480-cv.
- Decision date: 2008-08-04.
- Disposition: District-court judgment reversed, injunction vacated, and case
  remanded.
- As-of date: 2026-07-16.
- Counsel review: Not performed.

## Question Presented

Did a remote-storage DVR system directly infringe reproduction and public-
performance rights through short-lived buffers, subscriber-requested playback
copies, and transmissions from each subscriber's unique copy?

## Verified Holding And Record

The Second Circuit held that fixation under § 101 requires both embodiment in a
medium and embodiment for more than a transitory duration. Data in the system's
buffers could be reproduced or communicated and therefore satisfied the
embodiment requirement, but no bit remained for more than 1.2 seconds before
being automatically overwritten.

On those facts, the buffered works failed the duration requirement and were not
fixed copies. The court distinguished *MAI Systems Corp. v. Peak Computer, Inc.*
because the RAM representation there remained until the computer was turned off
and the duration issue had not independently been litigated.

The court did not establish 1.2 seconds as a universal legal threshold. It
repeatedly described the inquiry as fact-specific and left open whether other
technical facts could materially change the duration analysis.

For the persistent playback copies stored on the service's hard disks, the
court held that the subscribers, not the system operator, caused the copies for
direct-liability purposes. Subscribers selected the programs and issued the
recording commands. Designing, hosting, and maintaining an automated system did
not by itself make the operator the direct copier on the reviewed record.

The court separately held that playback transmissions from one subscriber's
unique copy to that same subscriber were not public performances under its
transmit-clause analysis. It expressly refused to create general immunity for
content-delivery systems and did not decide other reproduction or secondary-
liability theories.

## Limits And Unresolved Questions

- The buffer holding depends on both technical embodiment and duration,
  including overwrite behavior and the time each bit remains available.
- The opinion does not hold that every buffer, packet, register, cache, RAM
  state, or temporary file is legally transitory.
- It does not establish a bright-line number of seconds for fixation.
- The direct-liability result concerns volition or causation; it does not
  resolve contributory, vicarious, inducement, or other secondary liability.
- The plaintiffs expressly disavowed secondary liability, and the service
  operator waived fair use, so neither issue was decided.
- The court did not decide whether hypothetical buffer copies would be de
  minimis.
- The unique-copy public-performance analysis was expressly narrow and must be
  read with later Supreme Court and controlling circuit authority.
- Persistent user-requested recordings remained copies; the absence of direct
  operator liability did not make the copies nonexistent or authorized.
- Contract, licensing, access-control, and privacy questions remain separate.

## Repository Relevance

SHAR must record the exact lifetime and overwrite behavior of every buffer,
cache, decompressed representation, generated asset, and runtime state. A state
that can be reproduced is not automatically fixed, but describing it as
`temporary`, `in memory`, or `streaming` does not establish transitory duration.

Automated tooling also requires a separate actor analysis. A user command,
operator-authored default, scheduled process, server policy, or repository build
may supply different causation. Lack of direct volitional conduct never supplies
publication permission or eliminates possible secondary liability.

## Required Facts

- The work, material object, buffer, cache, RAM region, and persistent storage.
- The duration, overwrite process, stability, and ability to reproduce each
  state.
- Whether all or only a fragment of the work is embodied over time.
- The person or process that selects the work and triggers each copy.
- System defaults, operator intervention, automated rules, and user commands.
- Every persistent copy, recipient, transmission, and potential audience.
- Direct and secondary liability theories analyzed separately.
- Later controlling authority, licenses, statutory limitations, and defenses.

## Sources

- United States Court of Appeals for the Second Circuit (2008), *Cartoon Network
  LP, LLLP v. CSC Holdings, Inc.*, 536 F.3d 121, archived official opinion PDF.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://web.archive.org/web/20110429050646if_/http://www.ca2.uscourts.gov/decisions/isysquery/339edb6b-4e83-47b5-8caa-4864e5504e8f/1/doc/07-1480-cv_opn.pdf>
  (Accessed: 16 July 2026).
- [17 U.S.C. § 101](../statutes/17-usc-101.md).
- [17 U.S.C. § 106](../statutes/17-usc-106.md).
- [MAI Systems Corp. v. Peak Computer, Inc.](mai-systems-v-peak-computer.md).
- [Capitol Records, LLC v. ReDigi Inc.](capitol-records-v-redigi.md).
