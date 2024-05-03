"use client"

import * as React from "react"
import Link from "next/link"

import { cn } from "@/lib/utils"
import { ModeToggle } from "../mode-toggle";

import {
    NavigationMenu,
    NavigationMenuContent,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    NavigationMenuTrigger,
} from "@/components/ui/navigation-menu"

export function NavigationMenuComp({ title, onlyOptions = false }: { title?: string; onlyOptions?: boolean }) {
    return (
        <NavigationMenu className="z-20 mb-2 mt-2">
            <NavigationMenuList>
                <NavigationMenuItem>
                    <NavigationMenuTrigger>{title}</NavigationMenuTrigger>
                    <NavigationMenuContent>
                        <ul className="grid gap-3 p-6 w-[18rem] md:w-[400px] lg:w-[500px] lg:grid-cols-[.75fr_1fr]">
                            <li className="row-span-3">
                                <Link
                                    className="flex h-full w-full select-none flex-col justify-center rounded-md bg-gradient-to-b from-muted/50 to-muted p-6 no-underline outline-none focus:shadow-md"
                                    href="/"
                                >
                                    {!onlyOptions ?
                                        <>
                                            <div className="mb-2 mt-2 text-2xl font-medium">
                                                REnizer
                                            </div>
                                            <p className="text-sm leading-tight text-muted-foreground">
                                                Powering Your Green Initiatives - Your Renewable Energy Project Partner
                                            </p>
                                        </>
                                        : <div className="mb-2 mt-2 text-3xl font-medium">
                                            Go Home
                                        </div>
                                    }
                                </Link>
                            </li>
                            <ListItem href="/signup?type=manager" title="Manager">
                                Streamline Your Renewable Vision
                            </ListItem>
                            <ListItem href="/signup?type=contributor" title="Contributor">
                                Collaborate for Clean Energy
                            </ListItem>
                            <ListItem href="/signup?type=investor" title="Investor">
                                Invest in Tomorrow&apos;s Energy
                            </ListItem>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>
                <ModeToggle />
            </NavigationMenuList>
        </NavigationMenu>
    )
}

const ListItem = React.forwardRef<
    React.ElementRef<"a">,
    React.ComponentPropsWithoutRef<"a">
>(({ className, title, children, ...props }, ref) => {
    return (
        <li>
            <NavigationMenuLink asChild>
                <a
                    ref={ref}
                    className={cn(
                        "block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground",
                        className
                    )}
                    {...props}
                >
                    <div className="text-sm font-medium leading-none">{title}</div>
                    <p className="line-clamp-2 text-sm leading-snug text-muted-foreground">
                        {children}
                    </p>
                </a>
            </NavigationMenuLink>
        </li>
    )
})

ListItem.displayName = "ListItem"
