# Radal — Speeding up Ground Penetrating Radar (GPR) processing
The aim of `radal` is to quickly and accurately process GPR data.
In one command, most data can be processed in pre-set profiles or with custom filter settings, and batch modes allow for sequences of datasets to be processed with the same settings.
Radal is a high-performance ground-penetrating radar (GPR) processing software.
The aim is to make GPR data easy to process and handle.
Built in [rust](https://rust-lang.org/) with high focus on testing and performance, Radal may be for you if large data volumes and strange fileformats are common issues.


Much of the functionality has been inspired from the projects [RGPR](https://github.com/emanuelhuber/RGPR) and [ImpDAR](https://github.com/dlilien/ImpDAR); both of which are more mature projects.
For example, Radal currently only works on Malå (.rd3) and pulseEKKO (.dt1) radar formats.
For many uses, these will more likely be the tools for you!

Prior to Feb. 2026, this program was called `rsgpr`.


### Installation

#### Requirements
- `cargo` for installing rust projects
- `gdal` (optional, for sampling heights from DEMs). For Debian or derivatives, this means `gdal-bin`.
- `proj` (optional, for CRS support other than WGS84 UTM Zones). For Debian or derivatives, this means `proj-bin`.

Using cargo, `radal` can be installed from the repo (after installing the requirements):
```bash
cargo install --git https://github.com/erikmannerfelt/radal.git
```

with nix, the flake can be used without worrying about the requirements above:
```nix
{
  inputs = {
    radal.url = "github:erikmannerfelt/radal";
  };
}
```
or in an ephemeral shell:
```bash
nix shell github:erikmannerfelt/radal

```


### Simple usage
See the help page of `radal` for info on how to interact with the CLI:
```bash
radal -h
```

To toggle useful information on a file, the `-i` or `--info` argument shows the metadata and a summary of the location data:
```bash
radal -f DAT_001_A1.rd3 -i
```

Processing a file using the default processing profile:

```bash
radal -f DAT_001_A1.rd3 --default
```

The output will be a NetCDF file with the same name but an `.nc` suffix.
By default, the output is saved in the same directory as the input.
For more control, the output directory and/or filename can be controlled with `-o` or `--output`.

To process multiple files in "batch mode", provide a ["glob"](https://en.wikipedia.org/wiki/Glob_(programming)) pattern as the filename.
Optionally, for many sequential files, the `--merge` argument allows merging multiple files into one.
```bash
radal -f "data/*.rd3" --merge "10 min" --default -o output/
```

A rudimentary profile renderer is available with the `-r` argument.
This will be saved in the same location as the output file as a JPG if another filename is not given.
