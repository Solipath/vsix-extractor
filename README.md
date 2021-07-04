# VSIX Extractor
This is a somewhat crude program designed to extract vsix files that are downloaded as part of the vs_buildTools.exe executable's layout flag.

This program is used as part of Solipath's attempt to get a working Visual Studio C++ compiler without requiring administrative rights, and/or writing anything to the windows registry. The more specific goal with this is to compile Python, and other programming languages from source on Windows (this is what I'm planning on doing for any language that doesn't have an official distribution that is installed by decompressing a file and setting an environment variable).

### VS Build tool command I used to download VSIX files
vs_BuildTools.exe --wait --quiet --layout temp --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --lang en-US

### Example usage of VSIX Extractor (temp is folder containing VSIX files, buildTools is output directory)
vsix-extractor.exe temp buildTools

## How I'm decompressing VSIX Files (which may be a gross simplification of how it is supposed to work)
VSIX files are effectively just zip compressed files with some metadata, and folder/file naming conventions

Each VSIX file has a manifest.json file at the root. This file may or may not contain an attribute called extensionDir.

This extensionDir is used to state where the files should be copied to. Every case I've run into so far have the path prefixed with "[installdir]" which I replace with whatever the output directory is.

If the VSIX file does not have an extensionDir defined, I will take the contents of the "Contents" folder and copy it to the output directory.

The manifest.json file also sometimes has a Byte Order Mark (bom) prefix, so in order to read it, I have to strip it out.

Finally, all directories are URL Encoded, so any spaces become %20, "#" becomes %23, etc. (Note that the manifest.json extensionDir field is not encoded)

