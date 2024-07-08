# GFF3 validator

A GFF3 validator to prevalidate GFF3 files before conversion to EMBL flatfile format and ENA submission.

## Problem

ENA requires an EMBL flatfile to make annotation submissions (currently). These are validated using
ENA's online validation system (https://github.com/enasequence/sequencetools). GFF3 files must be converted
using tools like EMBLmyGFF3, which is a time-consuming process. However, when online validation fails, the
GFF3 file must then be fixed, and then reconverted, wasting valuable time. 

This tool aims to run the EMBL flatfile validation checks on the GFF3 file.

