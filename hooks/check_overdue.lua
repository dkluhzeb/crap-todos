return function(context)
    if not context.data then
        return context
    end

    local due = context.data.due_date
    local status = context.data.status

    if status == "done_allegedly" then
        context.data.overdue = false
        return context
    end

    if due and due ~= "" then
        local now = crap.util.date_now()
        context.data.overdue = due < now
    else
        context.data.overdue = false
    end

    return context
end
