# todo_app
This is a simple todo app written in rust inspired from [todo](https://github.com/sioodmy/todo)

https://github.com/joshua-sajeev/todo_app/assets/99077093/92daaf26-467e-4c4b-91a6-c46cdf8fc050

Usage: todo [COMMAND] [ARGUMENTS]

Example: todo list

Available commands:

    - list| -l

        lists all tasks

        Example: todo list

    - add [TASK/s] | -a [TASK/s]

        adds new task/s

        Example: todo add \"buy carrots\" grocery

    - rm [INDEX] | -r [INDEX]

        removes a task

        Example: todo rm 4

    - done [INDEX] | -d [INDEX]

        marks task as done

        Example: todo done 2 3 (marks second and third tasks as completed)

    - done-list

        lists all tasks that are marked done

    - sort | -s 

        sorts completed and uncompleted tasks

        Example: todo sort

    - clear | -c

        deletes all tasks

    - restore 

        restore recent backup after clear

    - help | --help | -h 

        this message

