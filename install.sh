#!/bin/sh
if which lua &> /dev/null; then
    echo "Lua is installed. Skipping.."
else
    echo "Lua is not installed."
    echo "Installing Lua.."
    if which apt &> /dev/null; then
    sudo apt-get install lua
    MNGR=apt-get
else
    echo "Assuming DNF/YUM.. if not, end the program now and install lua and luarocks manually."
    sleep 5
    MNGR=dnf
    sudo dnf install lua
fi
fi
if which luarocks &> /dev/null; then
    echo "Luarocks is installed. Skipping.."
else
    echo "Installing luarocks.."
    $MNGR install luarocks
fi
sudo luarocks install luafilesystem
# pretend that it copies the files omg omg (it doesn't)
echo "done"
exit 0