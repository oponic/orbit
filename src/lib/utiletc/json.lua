local json = {}
function json.encode(value)
    if type(value) == "table" then
        local result = {}
        for k, v in pairs(value) do
            table.insert(result, '"' .. tostring(k) .. '":' .. json.encode(v))
        end
        return "{" .. table.concat(result, ",") .. "}"
    elseif type(value) == "string" then
        return '"' .. value:gsub('"', '\\"') .. '"'
    elseif type(value) == "number" or type(value) == "boolean" then
        return tostring(value)
    else
        return 'null'
    end
end
function json.decode(jsonStr)
    local jsonStr = jsonStr:gsub("%s+", "")
    if jsonStr:sub(1, 1) == "{" then
        local result = {}
        jsonStr = jsonStr:sub(2, -2)
        for k, v in jsonStr:gmatch('("([^"]+)":)([^,}]+)') do
            result[k:sub(1, -2)] = json.decode(v)
        end
        return result
    elseif jsonStr:sub(1, 1) == '"' then
        return jsonStr:sub(2, -2):gsub('\\"', '"')
    elseif jsonStr == "null" then
        return nil
    elseif jsonStr == "true" then
        return true
    elseif jsonStr == "false" then
        return false
    else
        return tonumber(jsonStr) or jsonStr
    end
end
return json