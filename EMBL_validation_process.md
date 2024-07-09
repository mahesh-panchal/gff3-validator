# EMBL validation process

An attempt to trace through the EMBL code to figure out what kind of validations
happen on EMBL flatfiles.

## Start point is through the Webin-cli

Assuming the starting point is https://github.com/enasequence/sequencetools/blob/master/src/main/java/uk/ac/ebi/embl/api/validation/submission/SubmissionValidator.java

Should call the `validate` function (Line:89).

In SubmissionValidationPlan, `execute` function, Flatfiles are validated on Line:89, which leads to Line:233.

Flatfiles are then processed by an object `FlatfileFileValidationCheck`.
Each file in the submission process is then checked using the `check` function (Line:243).

The flatfile is validated with the method `valdiateFileFormat` in `FlatfileFileValidationCheck` (Line:49-51).

`validateFileFormat` is defined in the abstract class `FileValidationCheck` (Line:684).
Line:696 starts with a buffered file reader.

### Check 1

Line:698-702

If the empty line count at the start of a file exceeds 100 lines, then fail.

### Check 2

Line:704-710

If the line does not start with "ID", then fail.

There are no more checks to `validateFileFormat`.

Back to `FlatfileFileValidationCheck`. Line:62 makes a new `EmblEntryReader`. The constructor on Line:64-70, makes a new `EntryReader` with arg `new EmblLineReader`.
That in turn calls the constructor for `FlatFileEntryReader`. `EmblEntryReader` also adds several block readers.

- `IDReader`
- `ACReader`
- `DEReader`
- `KWReader`
- `DTReader`
- `PRReader`
- `ACStarReader`
- `STStarReader`
- `COReader`
- `SQReader`
- `AHReader`
- `FHReader`
- `CCReader`
- `DRReader`
- `OSReader`
- `OCReader`
- `OGReader`
- `ASReader`
- `RAReader`
- `RCReader`
- `RGReader`
- `RLReader`
- `RNReader`
- `RPReader`
- `RTReader`
- `RXReader`

These are all tags within an EMBL flat file.

Example:
```
ID   SAMPLE123; SV 1; linear; mRNA; STD; HUM; 270 BP.
XX
AC   X12345;
XX
DE   Sample Human mRNA for EMBL Flatfile Example
XX
KW   .
XX
OS   Homo sapiens (human)
OC   Eukaryota; Metazoa; Chordata; Craniata; Vertebrata; Euteleostomi; Mammalia;
OC   Eutheria; Primates; Haplorrhini; Catarrhini; Hominidae; Homo.
XX
RN   [1]
RP   1-270
RX   DOI; 10.1234/sample1234.
RA   Doe J., Smith A., Public P.;
RT   "Title of the study describing the mRNA";
RL   Journal Name. (YEAR) VOLUME: PAGES-PAGES.
XX
FH   Key             Location/Qual Tabies
FH
FT   source          1..270
FT                   /organism="Homo sapiens"
FT                   /mol_type="mRNA"
FT                   /db_xref="taxon:9606"
FT   mRNA            join(1..70, 71..150, 151..270)
FT                   /gene="SAMPLE"
FT                   /product="sample protein"
FT   CDS             1..270
FT                   /codon_start=1
FT                   /gene="SAMPLE"
FT                   /product="sample protein"
FT                   /db_xref="GOA:A12345"
FT                   /db_xref="InterPro:IPR000123"
FT                   /protein_id="P12345"
XX
SQ   Sequence 270 BP; 61 A; 66 C; 76 G; 67 T; 0 other;
     acgtacgtac gtacgtacgt acgtacgtac gtacgtacgt acgtacgtac gtacgtacgt        60
     gcatgcatgc atgcatgcat gcatgcatgc atgcatgcat gcatgcatgc atgcatgcat       120
     cgtacgtacg tacgtacgta cgtacgtacg tacgtacgtt gcatgcatgc atgcatgcat       180
     acgtacgtac gtacgtacgt acgtacgtac gtgcgtacgt acgtacgtac gtacgtacgt       240
     acgtacgtacgt                                                        270
//
```

There is no specific reader for the feature table block (`FT`) although there is one for the feature header (`FH`).

Line:63 of `FlatfileFileValidationCheck` then calls `FlatFileEntryReader::read()`. This calls `EntryReader::readLines()` (Line:129-208).
Line:142 calls `EmblEntryReader::readFeature()`, which checks if it's a `FT` tag, and adds a `FeatureReader`.

### Checks in FeatureReader

Line:54-149

Comment at top of section:
```
    // The feature names must appear in the correct position.
    // The feature qualifiers must appear in the correct position.
    // The feature locations must appear in the correct position.
    // The feature locations are terminated by a feature name or
    // a feature location.
    // The qualifier value must continue in the correct position.
    // The qualifier value can only continue if the qualifier is double
    // quoted.
```

There seem to be error codes:

#### FT.1 

`FeatureReader`, Line:154. 

Error if line.length is less than or equal to 21. (Data is after this indentation).

#### FT.12

`FeatureReader`, Line:160.

Error if a feature name (e.g. source, mRNA, CDS) contains a space.

#### FT.2

`FeatureReader`, Line:164

Error if there is no feature name.
The name is then checked against the list: `src/main/resources/uk/ac/ebi/embl/api/validation/data/feature-keys.tsv`, and the case (upper/lower) fixed if it matches anything on that list.

#### FT.17

`FeatureReader`, Line:170

The text following the feature name should be a location string (e.g., `1..125`).

#### FT.3

`FeatureReader`, Line:198

Error if location string is blank.

#### FT.8

`FeatureReader`, Line:215

Error if there's no text after 21st char (data is after this margin/indentation), after a location line

#### FT.4

`FeatureLocationParser`, Line:54

Error, if the location is not a valid compound location.
This might be the same as `FT.17`.

#### FT.15

`FeatureReader`, Line:177

Error if either start or end in a location are less than 1.

#### FT.16

`FeatureReader`, Line:181

Error if location implements the RemoteLocation interface and submission is happening through the webin-cli.
I'm not sure what this means. 

#### FT.5

`FeatureReader`, Line:243

Error if there is no text after position 21.

`FeatureReader`, Line:277

Error if the following line has no text after position 21.

#### FT.10

`QualifierMatcher`, Line:39-42

Error if the qualifier value is quoted, and there are characters outside of the quotation marks.

`QualifierMatcher`, Line:45-47

Error if there are quotes between the quotes.

#### FT.6

`FeatureReader`, Line:94

Error if the qualifier is "organism", and it's not an instance of SourceFeature (i.e. not part of source feature).

#### FT.14

`FeatureReader`, Line:111

Error if the qualifier is "mol_type", and it's not an instance of SourceFeature.

#### FT.7

`FeatureReader`, Line:119-121

Error if the mol_type doesn't match the sequence entry type (from ID?).

#### FT.6

`FeatureReader`, Line:131

Error if qualifier is "db_xref" and value is "taxon:Int", and it's not an instance of SourceFeature.

#### FT.9

`FeatureReader`, Line:145

Error if no "mol_type" in the SourceFeature