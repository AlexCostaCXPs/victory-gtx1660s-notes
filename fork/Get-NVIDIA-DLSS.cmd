REM VIctory: this file is part of VIctory, distributed under the GNU General Public License, version 3 or later (GPL-3.0-or-later). See the LICENSE file at the root of the repository.

@echo off
rem Downloads the NVIDIA files that DLSS and DLSS frame generation need (about 70 MB) from NVIDIA's own
rem GitHub and puts them where the launcher looks for them (nvidia-sdk\ in this folder). Run it once.
rem They are NVIDIA's files under NVIDIA's licences, which are saved beside them; this repository does
rem not contain them. Without them the game still runs, with its own upscaling and no frame generation.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0scripts\get-nvidia-sdks.ps1" %*
echo.
pause