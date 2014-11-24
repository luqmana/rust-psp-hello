A simple Rust Hello World to run on a PSP.

```
> % ./gen-target.sh
> % cargo build --target psp
> % ./gen-eboot.sh
```

This will give you an `EBOOT.PBP` which you can just copy over to your PSP and run as usual. Alternatively you can run the prx from pspsh.
