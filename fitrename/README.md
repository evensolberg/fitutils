# fitrename

This program will rename the input file(s) based on metadata from the files and a pattern supplied.

For example,

`fitrename *.fit -p "%year-%month-%day %hour.%minute.%second %activity %duration-minutes.%duration-seconds"`

will rename the file based on the date and time when the activity started, the activity name, and the duration in minutes and seconds. This may yield something like:

`2022-02-10 06.50.10 Indoor Rowing 30.04.fit`

## Command Line Flags

The application takes the following form:

`fitrename <FILE(S)> -p "pattern"`

Wildcards and mutiple file names are supported, eg.

`fitrename *.fit *.gpx -p "%year-%month-%day %hour.%minute.%second %activity %duration-minutes.%duration-seconds"`

|Flag|Required|Description|
|:---|:------:|:----------|
`-p`|Yes|File rename pattern, as described in the next section.

## Rename Tokens

The following tokens can be used. Note that date and times indicate the *start* of the activity:

|Token Long|Token Short|Description|
|:----|:----|:----------|
`%year`|`%yr`|The year.
`%month`|`%mo`|The month (01-12).
`%day`|`%dy`|The day (01-31).
`%weekday`|`%wd`|The day of the week (Mon, Tue, Wed, Thu, Fri, Sat, Sun).
`%hour`|`%h`|The hour (00-23).
`%hour24`|`%24`|The hour (00-23).
`%hour12`|`%12`|The hour (00-12).
`%minute`|`%mi`|The minute (00-59).
`%second`|`%se`|The second (00-59).
`%ampm`|`%ap`|Indicates whether the time is `AM` or `PM`.
`%activity`|`%ac`|The name of the activity, eg. "Running", "Walking" or "Cycling", etc.
`%activity_detailed`|`%ad`|The detailed part of the activity, eg "indoor_cycling", "spin" or "generic".
`%duration`|`%du`|The duration of the activity in seconds.
`%manufacturer`|`%mf`|The manufacturer of the product that crated the file, eg. "Garmin", "Wahoo".
`%product`|`%pr`|The product that created the file eg. "Fenix 7X".
`%serial_number`|`%sn -`|The product that created the file eg. "Fenix 7X".
