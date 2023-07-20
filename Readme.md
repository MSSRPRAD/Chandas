# Chandas

## Reason for Rewrite

This is a total re-write of my SanskritMetresRust project due to many reasons including:
1. The code was totally unreadable
2. There were no well defined structs
3. I had cloned the whole vidyut repo and was working in a subdirectory. Cloning it on my PC was taking too long.

## Setup Instructions (Assuming rust, cargo, python, pip are installed)

```
git clone https://github.com/MSSRPRAD/Chandas.git;
cd Chandas;
cargo run --release;
```

## Python bindings

```
virtualenv venv;
. venv/bin/activate.fish;
pip install maturin;
maturin develop;
python;
>>> import chandas
>>> from chandas import identify_anushtup_rs
>>> from chandas import identify_vrtta_rs
>>> from chandas import identify_matra_rs
```

## Demo
```
virtualenv venv;
. venv/bin/activate.fish;
pip install flask maturin;
python demo/app.py
```
