# Rote

## Abstract
Rote should be a note-taking tool that automates the process of creating files,
naming them, and organising thoughts within them into separate sections. It
should also make it easier search them, both by enforcing consistent
naming/dating schemes, and potentially by automating the process and even
implementing Regex.

------

## Conceptual Workflow (example)
1. Call rote from command-line
2. Provide name for note
3. Type note
4. Save & exit

## Desired Functionality
* Creating/appending to a file named using the current date; one file per day.
* Automatically writing out metadata in the file before your note.
* Opening vim so the user simply needs to type their note and exit.

## Stretch Goals
* Search files for a phrase.
* Allow for completely automated adding of short notes (think `git commit -m`).
