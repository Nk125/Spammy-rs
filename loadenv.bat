@echo off

if not exist "%~1" (
	set "fpath=.env"
) else (
	set "fpath=%~1"
)

if not exist %fpath% (
	echo .env file must exist
	exit /b 1
)

echo Loaded env variables:

for /f "usebackq tokens=*" %%a in (%fpath%) do (
	call :checkcomment "%%a"
	if not errorlevel 1 (
		set "%%a"
		call :trimprint "%%a"
	)
)

exit /b 0

:checkcomment
setlocal enabledelayedexpansion
set "str=%~1"
if not "!str!"=="" if not "!str:~0,1!"=="#" (
	endlocal & exit /b 0
)
endlocal & exit /b 1

:trimprint
setlocal enabledelayedexpansion
set "str=%~1"
if not defined str (
	endlocal & exit /b 1
)
set "key="
for /f "delims== tokens=1" %%i in ("!str!") do set "key=%%i"
echo #    !key!
endlocal & exit /b 0