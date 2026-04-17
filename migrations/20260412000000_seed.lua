local M = {}

function M.up()
    crap.log.info("Seeding Crap Todo data...")
    local opts = { overrideAccess = true }

    -- ── Users ────────────────────────────────────────────────────────────

    local admin = crap.collections.create("users", {
        email = "admin@craptodo.local",
        password = "crap123",
        name = "Crap Admin",
        bio = "I manage the chaos",
        role = "admin",
    }, opts)

    local larry = crap.collections.create("users", {
        email = "larry@craptodo.local",
        password = "crap123",
        name = "Lazy Larry",
        bio = "Professional procrastinator",
        role = "user",
    }, opts)

    crap.log.info("Created users: " .. admin.id .. ", " .. larry.id)

    -- ── Categories ───────────────────────────────────────────────────────

    local work = crap.collections.create("categories", {
        name = "Work Stuff",
        color = "#3b82f6",
        icon = "briefcase",
    }, opts)

    local chores = crap.collections.create("categories", {
        name = "Chores",
        color = "#22c55e",
        icon = "house",
    }, opts)

    local goals = crap.collections.create("categories", {
        name = "Life Goals",
        color = "#a855f7",
        icon = "rocket",
    }, opts)

    local dread = crap.collections.create("categories", {
        name = "Existential Dread",
        color = "#ef4444",
        icon = "skull",
    }, opts)

    crap.log.info("Created categories")

    -- ── Tasks ────────────────────────────────────────────────────────────

    local tasks = {
        {
            title = "Fix that bug from 3 months ago",
            description = "It's been in the backlog since January. The backlog is now the back-mountain.",
            status = "barely_started",
            priority = 5,
            due_date = "2026-03-01",
            assignee = larry.id,
            category = work.id,
            subtasks = {
                { title = "Reproduce the bug", done = false },
                { title = "Cry about it", done = true },
                { title = "Actually fix it", done = false },
            },
        },
        {
            title = "Learn Rust",
            description = "The borrow checker is not your enemy. It's your frenemy.",
            status = "kinda_doing_it",
            priority = 3,
            assignee = larry.id,
            category = goals.id,
            subtasks = {
                { title = "Read The Book", done = true },
                { title = "Fight the borrow checker", done = true },
                { title = "Win against the borrow checker", done = false },
            },
        },
        {
            title = "Do laundry",
            description = "The pile has achieved sentience.",
            status = "almost_done",
            priority = 2,
            due_date = "2026-04-13",
            assignee = larry.id,
            category = chores.id,
        },
        {
            title = "Touch grass",
            description = "Go outside. Feel the sun. Remember what trees look like.",
            status = "barely_started",
            priority = 1,
            category = goals.id,
            assignee = larry.id,
        },
        {
            title = "Reply to that email from February",
            description = "At this point a carrier pigeon would be faster.",
            status = "barely_started",
            priority = 4,
            due_date = "2026-02-15",
            assignee = admin.id,
            category = work.id,
        },
        {
            title = "Organize desktop icons",
            description = "There are 847 files on the desktop. This is fine.",
            status = "barely_started",
            priority = 2,
            assignee = admin.id,
            category = chores.id,
        },
        {
            title = "Find meaning in sprint retrospectives",
            description = "What went well: we survived. What could improve: everything.",
            status = "kinda_doing_it",
            priority = 3,
            assignee = admin.id,
            category = dread.id,
        },
        {
            title = "Write unit tests",
            description = "Past you was supposed to do this. Past you was irresponsible.",
            status = "barely_started",
            priority = 4,
            due_date = "2026-04-01",
            assignee = larry.id,
            category = work.id,
            subtasks = {
                { title = "Figure out what to test", done = false },
                { title = "Write the tests", done = false },
                { title = "Fix the tests", done = false },
                { title = "Fix what the tests found", done = false },
            },
        },
        {
            title = "Cancel unused subscriptions",
            description = "You're paying for 3 streaming services you haven't opened since 2024.",
            status = "done_allegedly",
            priority = 2,
            assignee = larry.id,
            category = chores.id,
        },
        {
            title = "Question all life choices",
            description = "Scheduled for every Sunday at 3am.",
            status = "kinda_doing_it",
            priority = 5,
            category = dread.id,
            assignee = admin.id,
        },
    }

    for _, task in ipairs(tasks) do
        crap.collections.create("tasks", task, opts)
    end

    crap.log.info("Created " .. #tasks .. " tasks")

    -- ── Comments ─────────────────────────────────────────────────────────

    local all_tasks = crap.collections.find("tasks", {
        limit = 3,
        overrideAccess = true,
    })

    if all_tasks and all_tasks.docs then
        for i, doc in ipairs(all_tasks.docs) do
            crap.collections.create("comments", {
                body = i == 1 and "Have we tried turning it off and on again?"
                    or i == 2 and "I too am in this picture and I don't like it."
                    or "This has been on the board for so long it pays rent.",
                author = (i % 2 == 0) and larry.id or admin.id,
                task = doc.id,
            }, opts)
        end
        crap.log.info("Created comments")
    end

    -- ── Globals ──────────────────────────────────────────────────────────

    crap.globals.update("app_settings", {
        app_name = "Crap Todo",
        tagline = "Because your tasks deserve better than a JSON file",
        motd = "Remember: done is better than perfect. But also, is anything really done?",
    }, opts)

    crap.log.info("Seed complete!")
end

function M.down()
    local opts = { overrideAccess = true }

    crap.collections.delete_many("comments", { overrideAccess = true })
    crap.collections.delete_many("tasks", { overrideAccess = true })
    crap.collections.delete_many("categories", { overrideAccess = true })
    crap.collections.delete_many("users", { overrideAccess = true })
end

return M
