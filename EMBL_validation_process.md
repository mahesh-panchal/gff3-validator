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
