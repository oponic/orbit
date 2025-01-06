local SaveSystem = {}
local function serialize(val, name, skipnewlines, depth)
    skipnewlines = skipnewlines or false
    depth = depth or 0

    local tmp = string.rep(" ", depth)

    if name then
        tmp = tmp .. name .. " = "
    end

    if type(val) == "table" then
        tmp = tmp .. "{" .. (not skipnewlines and "\n" or "")

        for k, v in pairs(val) do
            tmp = tmp .. serialize(v, k, skipnewlines, depth + 1) .. "," 
                     .. (not skipnewlines and "\n" or "")
        end

        tmp = tmp .. string.rep(" ", depth) .. "}"
    elseif type(val) == "number" then
        tmp = tmp .. tostring(val)
    elseif type(val) == "string" then
        tmp = tmp .. string.format("%q", val)
    elseif type(val) == "boolean" then
        tmp = tmp .. (val and "true" or "false")
    else
        tmp = tmp .. "\"[" .. type(val) .. "]\""
    end

    return tmp
end
function SaveSystem.save(saveName, data)
    local fullPath = saveName .. ".orbit"
    local file = io.open(fullPath, "w")
    if not file then
        return false, "Could not open file for writing"
    end
    local serialized = "return " .. serialize(data)
    file:write(serialized)
    file:close()
    return true
end
function SaveSystem.load(saveName)
    local fullPath = saveName .. ".orbit"
    local file = io.open(fullPath, "r")
    
    if not file then
        return nil, "Could not open file for reading"
    end
    local content = file:read("*a")
    file:close()
    local fn, err = load(content)
    if not fn then
        return nil, "Could not parse save file: " .. (err or "unknown error")
    end
    local success, result = pcall(fn)
    if not success then
        return nil, "Could not load save file: " .. (result or "unknown error")
    end
    return result
end
return SaveSystem
