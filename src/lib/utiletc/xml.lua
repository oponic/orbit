local xml = {}
function xml.encode(value)
    local result = {}
    if type(value) == "table" then
        for k, v in pairs(value) do
            table.insert(result, "<" .. k .. ">" .. xml.encode(v) .. "</" .. k .. ">")
        end
    else
        return tostring(value)
    end
    return table.concat(result)
end
function xml.decode(xmlStr)
    local result = {}
    for k, v in xmlStr:gmatch("<(%w+)>(.-)</%w+>") do
        result[k] = xml.decode(v)
    end
    return result
end
return xml