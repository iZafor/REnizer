"use client";

import { cn } from "@/lib/utils";
import React, { Ref } from "react";

export default function RetroGrid({ className, ref, children }: { className?: string, ref?: Ref<HTMLDivElement>, children?: React.ReactNode }) {
    return (
        <div
            className={cn(
                "pointer-events-none absolute h-full w-full overflow-hidden opacity-50 [perspective:200px]",
                className,
            )}
            ref={ref}
        >
            {/* Grid */}
            <div className="absolute inset-0 [transform:rotateX(15deg)]">
                <div
                    className={cn(
                        "animate-grid",

                        "[background-repeat:repeat] [background-size:60px_60px] [height:300vh] [inset:0%_0px] [margin-left:-50%] [transform-origin:100%_0_0] [width:600vw]",

                        // Light Styles
                        "[background-image:linear-gradient(to_right,rgba(0,0,0,0.3)_1px,transparent_0),linear-gradient(to_bottom,rgba(0,0,0,0.3)_1px,transparent_0)]",

                        // Dark styles
                        "dark:[background-image:linear-gradient(to_right,rgba(255,255,255,0.6)_2px,transparent_0),linear-gradient(to_bottom,rgba(255,255,255,0.6)_2px,transparent_0)]",
                    )}
                />
                {children}
            </div>

            {/* Background Gradient */}
            <div className="absolute inset-0 bg-gradient-to-t from-white to-transparent to-90% dark:from-gray-800" />
        </div>
    );
}
