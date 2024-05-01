"use client";

import { AnimatedBeam } from "../magicui/animated-beam";
import React, { forwardRef, useRef } from "react";
import { FcManager } from "react-icons/fc";
import Monitoring from "../icons/monitoring";
import { cn } from "@/lib/utils";
import Image from "next/image";
import TooltipWrapper from "./tooltip-wrapper";

const Circle = forwardRef<
    HTMLDivElement,
    { className?: string; children?: React.ReactNode }
>(({ className, children }, ref) => {
    return (
        <div
            ref={ref}
            className={cn(
                "z-10 dark:bg-gray-800 bg-white flex size-14 items-center justify-center rounded-full border-2 p-3 shadow-[0_0_20px_-12px_rgba(0,0,0,0.8)]",
                className,
            )}
        >
            {children}
        </div>
    );
});

Circle.displayName = "Circle";

export default function StakeholdersBeam() {
    const containerRef = useRef<HTMLDivElement>(null);
    const div1Ref = useRef<HTMLDivElement>(null);
    const div2Ref = useRef<HTMLDivElement>(null);
    const div3Ref = useRef<HTMLDivElement>(null);
    const div6Ref = useRef<HTMLDivElement>(null);
    const div7Ref = useRef<HTMLDivElement>(null);

    return (
        <div
            className="relative flex h-full w-full max-w-[40%] items-center justify-center overflow-hidden rounded-lg bg-transparent p-10 md:shadow-xl"
            ref={containerRef}
        >
            <div className="flex h-full w-full flex-row items-stretch justify-between gap-10">
                <div className="flex flex-col justify-center gap-10">
                    <Circle ref={div1Ref}>
                        <TooltipWrapper tooltip="Contributors">
                            <Image
                                src="/images/employees.png"
                                alt="employees"
                                height={128}
                                width={128}
                            />
                        </TooltipWrapper>
                    </Circle>
                    <Circle ref={div2Ref}>
                        <TooltipWrapper tooltip="Managers">
                            <FcManager size={30} className="text-black" />
                        </TooltipWrapper>
                    </Circle>
                    <Circle ref={div3Ref}>
                        <TooltipWrapper tooltip="Investors">
                            <Image
                                src="/images/investor.png"
                                alt="employees"
                                height={128}
                                width={128}
                            />
                        </TooltipWrapper>
                    </Circle>
                </div>
                <div className="flex flex-col justify-center">
                    <Circle ref={div6Ref} className="size-[8rem]">
                        <p className="text-2xl font-semibold">REnizer</p>
                    </Circle>
                </div>
                <div className="flex flex-col justify-center">
                    <Circle ref={div7Ref}>
                        <Monitoring />
                    </Circle>
                </div>
            </div>

            <AnimatedBeam
                containerRef={containerRef}
                fromRef={div1Ref}
                toRef={div6Ref}
                pathWidth={4}
            />
            <AnimatedBeam
                containerRef={containerRef}
                fromRef={div2Ref}
                toRef={div6Ref}
                pathWidth={4}
            />
            <AnimatedBeam
                containerRef={containerRef}
                fromRef={div3Ref}
                toRef={div6Ref}
                pathWidth={4}
            />

            <AnimatedBeam
                containerRef={containerRef}
                fromRef={div6Ref}
                toRef={div7Ref}
                pathWidth={4}
            />

        </div>
    );
}