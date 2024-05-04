"use client";

import { ModeToggle } from "@/components/ui/mode-toggle"
import { LuBell, LuSearch } from "react-icons/lu";
import Image from "next/image";
import Link from "next/link";
import { useTheme } from "next-themes";
import { Input } from "../input";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"

export default function Navbar() {
    const { theme } = useTheme();

    return (
        <header className="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <nav className="flex items-center justify-between p-4">
                <Link href="/dashboard" className="flex items-center">
                    <Image alt="RENIZER" height={64} width={64} src={theme == "light" ? "/images/logo-black.png" : "/images/logo-white.png"} />
                    <span className="font-bold">REnizer</span>
                </Link>
                <div className="flex items-center space-x-4">
                    <div className="flex items-center space-x-2 border-2 px-2 rounded">
                        <Input
                            className="focus-visible:ring-0 focus-visible:ring-offset-0 border-none mmd:w-[10rem]"
                            placeholder="Search here..."
                        />
                        <LuSearch />
                    </div>
                    <LuBell />
                    <Avatar>
                        <AvatarImage src="https://github.com/shadcn.png" />
                        <AvatarFallback>CN</AvatarFallback>
                    </Avatar>
                    <ModeToggle />
                </div>
            </nav>
        </header>
    )
}