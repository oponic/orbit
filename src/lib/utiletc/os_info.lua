local os_info = {}
local function is_windows()
    return package.config:sub(1, 1) == '\\'
end
function os_info.get_version()
    if is_windows() then
        return io.popen("ver"):read("*l")
    else
        return io.popen("uname -r"):read("*l") 
    end
end
function os_info.get_architecture()
    if is_windows() then
        return io.popen("wmic os get osarchitecture"):read("*l"):gsub("OS Architecture%s*:", ""):gsub("^%s*(.-)%s*$", "%1") 
    else
        return io.popen("uname -m"):read("*l") 
    end
end
function os_info.get_uptime()
    if is_windows() then
        local file = io.popen("wmic os get LastBootUpTime"):read("*l") 
        return os.time() - os.time({
            year = tonumber(file:sub(1, 4)),
            month = tonumber(file:sub(5, 6)),
            day = tonumber(file:sub(7, 8)),
            hour = tonumber(file:sub(9, 10)),
            min = tonumber(file:sub(11, 12)),
            sec = tonumber(file:sub(13, 14))
        }) 
    else
        local file = io.open("/proc/uptime", "r") 
        if file then
            local uptime = file:read("*n") 
            file:close()
            return uptime
        else
            return nil
        end
    end
end
return os_info