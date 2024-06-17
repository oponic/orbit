local function file_exists(name)
    local f = io.open(name, "r")
    if f then
        f:close()
        return true
    else
        return false
    end
end
local function check_structure()
    local required_structure = {
        "manifest.xml",
        "assets/",
        "assets/thumb256.png",
        "assets/thumb64.png",
        "config/",
        "data/"
    }
    for _, path in ipairs(required_structure) do
        if not file_exists(path) then
            return false
        end
    end
    return true
end
check_structure()
