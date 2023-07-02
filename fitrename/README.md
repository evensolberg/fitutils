# fitrename

This program will rename the input file(s) based on metadata from the files and a pattern supplied.

For example,

`fitrename *.fit -p "%year-%month-%day %hour.%minute.%second %activity %duration"`

will rename the file based on the date and time when the activity started, the activity name, and the duration in seconds. This may yield something like:

`2022-02-10 06.50.10 Indoor Rowing 1200.fit`

## Command Line Flags

The application takes the following form:

`fitrename <FILE(S)> -p "pattern"`

Wildcards and multiple file names are supported, eg.

`fitrename *.fit *.gpx -p "%year-%month-%day %hour.%minute.%second %activity"`

|Flag|Required|Description |
|:---|:------:|:----------|
`-p`|Yes|File rename pattern, as described in the next section.

## Rename Tokens

The following tokens can be used. Note that date and times indicate the *start* of the activity:

|Token Long|Token Short|FIT|GPX|TCX|Description|
|:----|:----|:---:|:---:|:---:|:----------|
`%year`|`%yr`|Y|Y|Y|The year.
`%month`|`%mo`|Y|Y|Y|The month (01-12).
`%day`|`%dy`|Y|Y|Y|The day (01-31).
`%weekday`|`%wd`|Y|Y|Y|The day of the week (Mon, Tue, Wed, Thu, Fri, Sat, Sun).
`%hour`|`%hr`|Y|Y|Y|The hour (00-23).
`%hour24`|`%24`|Y|Y|Y|The hour (00-23).
`%hour12`|`%12`|Y|Y|Y|The hour (00-12).
`%minute`|`%mi`|Y|Y|Y|The minute (00-59).
`%second`|`%se`|Y|Y|Y|The second (00-59).
`%ampm`|`%ap`|Y|Y|Y|Indicates whether the time is `AM` or `PM`.
`%activity`|`%ac`|Y| | |The name of the activity, eg. "Running", "Walking" or "Cycling", etc.
`%activity_detailed`|`%ad`|Y| | |The detailed part of the activity, eg "indoor_cycling", "spin" or "generic".
`%duration`|`%du`|Y|Y|Y|The duration of the activity in seconds.
`%manufacturer`|`%mf`|Y| | |The manufacturer of the product that crated the file, eg. "Garmin", "Wahoo".
`%product`|`%pr`|Y| | |The product that created the file eg. "Fenix 7X".
`%serial_number`|`%sn -`|Y|P *| |The product that created the file eg. "Fenix 7X".

* Note that for `%serial_number` some GPX files may have this in notes, and the application will attempt to extract a value.

> **NOTE:** Not all file types contain all of this information. Notably, FIT tends to be the most data-rich. You should do a dry run before attempting to rename files to ensure you get the expected result.

If a file before attempting to rename a file, the application will check if the target already exists. If it does, a unique identifier will be appended thusly: `filename (unique_id)` where `filename` is the duplicate name.
