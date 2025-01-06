local async = {}
function async.wrap(fn)
    return coroutine.create(function(...)
        return fn(...)
    end)
end
function async.await(coro)
    local status, result = coroutine.resume(coro)
    if not status then
        error(result)
    end
    return result
end
function async.run(fn, ...)
    local coro = async.wrap(fn)
    return async.await(coro)
end
return async