crap.collections.define("media", {
    labels = {
        singular = "Media",
        plural = "Media",
    },
    timestamps = true,
    upload = {
        mime_types = { "image/*" },
        max_file_size = "10MB",
        image_sizes = {
            { name = "thumbnail", width = 200, height = 200, fit = "cover" },
        },
    },
    admin = {
        use_as_title = "filename",
        default_sort = "-created_at",
        list_searchable_fields = { "filename" },
    },
    fields = {
        crap.fields.text({
            name = "alt",
            admin = { placeholder = "Alt text for accessibility" },
        }),
    },
    access = {
        read = "access.anyone",
        create = "access.authenticated",
        update = "access.authenticated",
        delete = "access.admin_only",
    },
})
