# Rust
# Rust VSCode
# Rust Jupyter
```shell
# MiniConda
wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
bash Miniconda3-latest-Linux-x86_64.sh
~/miniconda3/bin/conda init
conda config --set auto_activate_base false

# Jupyter
conda create -n rust python=3.10
conda activate rust
conda install jupyterlab -y
conda install nodejs -y 


cargo install evcxr_jupyter --version "0.15.0" --locked (for rustc 1.72)
evcxr_jupyter --install

jupyter lab --no-browser --ip 0.0.0.0 --no-browse
```