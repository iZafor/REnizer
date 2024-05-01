"use client";

import {
    Drawer,
    DrawerContent,
    DrawerTrigger,
} from "@/components/ui/drawer";
import { cn } from "@/lib/utils";
import { useTheme } from "next-themes";
import { CSSProperties } from "react";
import { Input } from "@material-tailwind/react";
import ShimmerButton from "../magicui/shimmer-button";

export default function LoginForm() {
    const { theme } = useTheme();
    const isLightTheme = () => theme === "light";

    return (
        <Drawer>
            <DrawerTrigger>
                <span
                    style={
                        {
                            "--spread": "90deg",
                            "--shimmer-color": isLightTheme() ? "hsl(var(--primary))" : "#ffffff",
                            "--radius": "1rem",
                            "--speed": "3s",
                            "--cut": "0.15em",
                            "--bg": isLightTheme() ? "hsl(var(--primary))" : "rgba(0, 0, 0, 1)",
                        } as CSSProperties
                    }
                    className={cn(
                        "group relative z-0 flex cursor-pointer items-center justify-center overflow-hidden whitespace-nowrap border border-white/10 px-6 py-3 text-gray-50 [background: var(--bg)] [border-radius:var(--radius)]",
                        "transform-gpu transition-transform duration-300 ease-in-out active:translate-y-[1px]",
                    )}
                >
                    <div
                        className={cn(
                            "-z-30 blur-[2px]",
                            "absolute inset-0 overflow-visible [container-type:size]",
                        )}
                    >
                        <div className="absolute inset-0 h-[100cqh] animate-slide [aspect-ratio:1] [border-radius:0] [mask:none]">
                            <div className="animate-spin-around absolute inset-[-100%] w-auto rotate-0 [background:conic-gradient(from_calc(270deg-(var(--spread)*0.5)),transparent_0,var(--shimmer-color)_var(--spread),transparent_var(--spread))] [translate:0_0]" />
                        </div>
                    </div>

                    Login

                    <div
                        className={cn(
                            "insert-0 absolute h-full w-full",
                            "rounded-2xl px-4 py-1.5 text-sm font-medium shadow-[inset_0_-8px_10px_#ffffff1f]",
                            "transform-gpu transition-all duration-300 ease-in-out",
                            "group-hover:shadow-[inset_0_-6px_10px_#ffffff3f]",
                            "group-active:shadow-[inset_0_-10px_10px_#ffffff3f]",
                        )}
                    />

                    <div
                        className={cn(
                            "absolute -z-20 [background:var(--bg)] [border-radius:var(--radius)] [inset:var(--cut)]",
                        )}
                    />
                </span>
            </DrawerTrigger>
            <DrawerContent className="h-[20rem]">
                <span className="w-full h-full grid content-start justify-center mt-8 pt-4">
                    <form className="w-[20rem] grid place-items-center space-y-4">
                        <Input label="Email" color={isLightTheme() ? "black" : "white"} required />
                        <Input type="password" label="Password" color={isLightTheme() ? "black" : "white"} required />
                        <ShimmerButton type="submit" borderRadius="1rem">Login</ShimmerButton>
                    </form>
                </span>
            </DrawerContent>
        </Drawer>
    )
}