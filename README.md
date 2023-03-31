

Open file:
```
code $HOME/.bashrc
```

Set environmental variables and close the file:
```
export AZURE_DEVOPS_PAT=""
export AZURE_DEVOPS_ORGANIZATION=""
export AZURE_DEVOPS_PROJECT=""
export AZURE_DEVOPS_API_VERSION=""
```

Apply configurations:
```
source $HOME/.bashrc
```

----

Build and execute project:
```
cargo run
```

Only execute project:
```
./target/debug/azureapirust
```