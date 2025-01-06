local uuid = {}
function uuid.generate()
    local template = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx"
    return template:gsub("x", function()
        return string.format("%x", math.random(0, 0xf))
    end):gsub("y", function()
        return string.format("%x", math.random(8, 0xb))
    end)
end
return uuid