# Linking


## Manual
```bash
#in root of project 
cc -c src/num.c
# First cc would generate lib.o but rustc needs a shared obejct, which is generated from
cc -shared num.o -o libnum.so

# Now we can link it to our Rust code
rustc -l num -L . src/bin/num.rs

# run the binary
./num
```

## Use build.rs

Add `build.rs` and use `cc` build-dep to compile c code and then link it as a static file to our rust binary.
