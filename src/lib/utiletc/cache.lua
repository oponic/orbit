local Cache = {}
Cache.__index = Cache
function Cache:new()
    local instance = setmetatable({}, self)
    instance.store = {}
    return instance
end
function Cache:set(key, value)
    self.store[key] = value
end
function Cache:get(key)
    return self.store[key]
end
function Cache:has(key)
    return self.store[key] ~= nil
end
function Cache:clear()
    self.store = {}
end
return Cache