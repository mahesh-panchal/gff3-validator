# GFF3 validator

> [!IMPORTANT]  
> This tool is still in development.

A GFF3 validator to prevalidate GFF3 files before conversion to EMBL flatfile format and ENA submission.

The GFF3 file is validated against a JSON schema. A schema following https://www.ebi.ac.uk/ena/WebFeat/
is in `schema/embl_webin_schema.json` (!WIP).

Usage:
```
gff3-validator <gff3_file> <json_schema>
```

## Problem

ENA requires an EMBL flatfile to make annotation submissions (currently). These are validated using
ENA's online validation system (https://github.com/enasequence/sequencetools). GFF3 files must be converted
using tools like EMBLmyGFF3, which is a time-consuming process. However, when online validation fails, the
GFF3 file must then be fixed, and then reconverted, wasting valuable time. 

This tool aims to run the EMBL flatfile validation checks on the GFF3 file.

