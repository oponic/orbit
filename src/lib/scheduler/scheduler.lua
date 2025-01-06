local Scheduler = {}
Scheduler.tasks = {}
function Scheduler:delay(func, delay)
    local task = {func = func, time = os.clock() + delay, once = false}
    table.insert(self.tasks, task)
end
function Scheduler:after(func, afterFunc)
    local task = {func = func, after = afterFunc, once = false}
    table.insert(self.tasks, task)
end
function Scheduler:once(func)
    local task = {func = func, once = true}
    table.insert(self.tasks, task)
end
function Scheduler:timeWindow(func, startTime, endTime)
    local task = {func = func, startTime = startTime, endTime = endTime, once = false}
    table.insert(self.tasks, task)
end
function Scheduler:update()
    local currentTime = os.clock()
    for i = #self.tasks, 1, -1 do
        local task = self.tasks[i]
        if task.time and currentTime >= task.time then
            task.func()
            table.remove(self.tasks, i)
        end
        if task.after and task.after() then
            task.func()
            table.remove(self.tasks, i)
        end
        if task.startTime and task.endTime then
            if currentTime >= task.startTime and currentTime <= task.endTime then
                task.func()
                if task.once then
                    table.remove(self.tasks, i)
                end
            end
        end
        if task.once then
            task.func()
            table.remove(self.tasks, i)
        end
    end
end
return Scheduler