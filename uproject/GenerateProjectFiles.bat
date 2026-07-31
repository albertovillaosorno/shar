@rem Copyright (c) 2026 Alberto Villa Osorno.
@echo off
setlocal
python "%~dp0Scripts\repair_unreal_project.py" %*
exit /b %errorlevel%
