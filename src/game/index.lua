local titleScreen = {}
function titleScreen:show()
    local ui = orbit_egui.ui
    ui:vertical(function(ui)
        ui:heading("heading")
        ui:add_space(20.0)
        if ui:button("test") then
        end
        ui:add_space(10.0)
        if ui:button("test2") then
        end
        ui:add_space(10.0)
        if ui:button("Exit") then
        end
    end)
end
return titleScreen