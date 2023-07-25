class Task:
    def __init__(self, name, description):
        self.name = name
        self.description = description
        self.status = "unassigned"

    def assign(self):
        self.status = "assigned"

    def complete(self):
        self.status = "completed"


class Developer:
    def __init__(self, name):
        self.name = name
        self.tasks = []

    def add_task(self, task):
        task.assign()
        self.tasks.append(task)


class SoftwareArchitect(Developer):
    def assign_tasks(self, tasks, developers):
        for i, task in enumerate(tasks):
            developers[i % len(developers)].add_task(task)


class Tester(Developer):
    def test_task(self, task):
        # Placeholder for testing code
        pass


class ProjectManager:
    def __init__(self, name):
        self.name = name
        self.tasks = []

    def create_task(self, name, description):
        task = Task(name, description)
        self.tasks.append(task)
        return task
