# Dependencies 

## For Linux
```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## For Windows
```sh
winget install cmake
```
install Visual Studio with C++ tools using instructions in 
https://visualstudio.microsoft.com/downloads/
( You can use code editor of your liking this is just to ensure installation of required C++ build tools, compilers and libraries )

install Rust using instructions in 
https://www.rust-lang.org/tools/install

You might need to restart to complete installation

# Build
```sh
git clone https://www.github.com/nandan-varma/notnice.git
# clone repo

cd notnice
# set cwd

cargo build
# build project

```