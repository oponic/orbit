local lua_popup = require("popup") -- im assuming htis is how lua_popup is defined
local lcatch = {}
function lcatch.try(func, ...)
    local start_time = os.clock()
    local success, result = pcall(func, ...)
    local end_time = os.clock()
    local execution_time = end_time - start_time
    if not success then
        lua_popup.popup:show_error("Error: " .. result)
    else
        lua_popup.popup:show_info("Execution time: " .. execution_time .. " seconds")
    end
    return result
end
return lcatch