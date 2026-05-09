Author: Ali Bukhamseen


# rustcounter  a Rust Linux Kernel Counter Module

`rustcounter` is a Linux kernel module that is written in Rust that creates a character device at `/dev/rustcounter`.

Each write to the device increments a kernel-space counter.  
Each read returns the current value of the counter.


---

## Features

- Rust Linux kernel module
- Character device at `/dev/rustcounter`
- Shared kernel-space state
- Read/write support
- Kernel logging with `dmesg`
- Built using Rust-for-Linux APIs

---

## Build

```bash
make clean
make
```

---

## Load Module

```bash
sudo insmod rustcounter.ko
```

---

## Verify Device

```bash
ls -la /dev/rustcounter
```

---

## Example Usage

```bash
sudo cat /dev/rustcounter
```

Output:

```text
0
```

Increment:

```bash
echo bump | sudo tee /dev/rustcounter > /dev/null
```

Read again:

```bash
sudo cat /dev/rustcounter
```

Output:

```text
1
```

---

## Kernel Logs

```bash
sudo dmesg | tail -10
```

Example:

```text
rustcounter: incremented to 1
rustcounter: read count 1
```

---

## Unload Module

```bash
sudo rmmod rustcounter
```

---

## Design Notes

This module uses:
- `MiscDeviceRegistration`
- `read_iter` / `write_iter`
- Rust-for-Linux APIs
- kernel-space shared state protected through Rust abstractions

The module demonstrates:
- safe kernel-space Rust programming
- Linux character devices
- Rust kernel module compilation
- user/kernel communication through `/dev`

---

## License

GPL-2.0
