'use client';

import {invoke} from "@tauri-apps/api";

type Project = {
    name: string,
    location: string,
    git_support: boolean,
    has_remote: boolean,
    origin_remote?: string
}

export default function Projects() {
    invoke<Project[]>("find_projects").then(console.log);
    return (
       <div>Test</div>
    );
}