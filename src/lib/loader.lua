local lfs = require("lfs")
local loader = {}
local function requireAll(path)
    for file in lfs.dir(path) do
        if file ~= "." and file ~= ".." then
            local fullPath = path .. "/" .. file
            local attr = lfs.attributes(fullPath)
            if attr.mode == "directory" then
                requireAll(fullPath)
            elseif file:match("%.lua$") then
                local moduleName = fullPath:gsub("/", "."):gsub("%.lua$", "")
                loader[moduleName] = require(moduleName)
            end
        end
    end
end
requireAll(CONFIG .. "/plugins/lib")
return loader