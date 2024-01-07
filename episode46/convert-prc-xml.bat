@echo off

for %%I in (%*) do (
    if "%%~xI"==".xml" (
        param-xml asm "%%I" -o "%%~nI" -l ParamLabels.csv
    ) else (
        param-xml disasm "%%I" -o "%%I.xml" -l ParamLabels.csv
    )
)
