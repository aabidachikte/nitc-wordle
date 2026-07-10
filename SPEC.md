# Wordle Word-Sourcing Architecture

## Overview

Replace the current hardcoded word array with a dynamic fetch from the NITC Wiki's Random API, applying a client-side filtering pipeline, with a safe fallback when the remote source is unavailable.

## Data Source

A GET request is made to the MediaWiki Random API:

https://wiki.fosscell.org/api.php?action=query&list=random&rnnamespace=0&rnlimit=20&format=json&origin=*

The endpoint returns up to 20 random page titles from the main (article) namespace.

## Filtering Pipeline

Incoming page titles are filtered client-side using two constraints applied in sequence:

1. **Regex**: `^[a-zA-Z]+$` — only titles consisting entirely of ASCII letters pass.
2. **Length**: title must have a length between 5 and 8 characters (inclusive).

Titles that fail either check are discarded before the word pool is built.

## Fallback Mechanism

If the API request fails (network error, non-200 response, or malformed reply) or returns zero valid campus words after filtering, the engine falls back to a safe, pre-defined local array of exactly 20 hardcoded words.

## Licensing

This project is officially governed by the **MIT License**.
