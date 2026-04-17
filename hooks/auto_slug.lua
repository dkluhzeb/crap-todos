return function(value, context)
    if value and value ~= "" then
        return value
    end

    local title = context.data and context.data.title
    if not title or title == "" then
        return value
    end

    return crap.util.slugify(title)
end
