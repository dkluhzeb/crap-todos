return function(context)
    if not context.user then
        return false
    end

    if context.user.role == "admin" then
        return true
    end

    if context.id and context.id == context.user.id then
        return true
    end

    return false
end
