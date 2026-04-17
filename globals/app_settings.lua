crap.globals.define("app_settings", {
    labels = { singular = "App Settings" },
    fields = {
        crap.fields.text({
            name = "app_name",
            required = true,
            default_value = "Crap Todo",
        }),
        crap.fields.text({
            name = "tagline",
            default_value = "Because your tasks deserve better than a JSON file",
        }),
        crap.fields.textarea({
            name = "motd",
            admin = {
                rows = 3,
                description = "Message of the day — shown on the dashboard",
                placeholder = "Something inspirational (or not)",
            },
        }),
    },
    access = {
        read = "access.anyone",
        update = "access.admin_only",
    },
})
