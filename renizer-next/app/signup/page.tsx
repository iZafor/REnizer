"use client";

import { useSearchParams, redirect } from "next/navigation";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import UserTypeMenu from "@/components/ui/home/user-type-menu";
import {
    Card,
    CardContent,
} from "@/components/ui/card";
import { CheckBoxWithLabel } from "@/components/ui/checkbox";
import { useState } from "react";

export default function Page() {
    const params = useSearchParams();
    const userType = params.get("type");
    if (!userType || !(["manager", "contributor", "investor"].some(t => t === userType))) redirect("/");

    return (
        <span className="w-full h-full grid place-items-center">
            <UserTypeMenu title="Choose Your Role" onlyOptions={true} />
            <Card className="msm:mt-10 mt-20">
                <CardContent>
                    <form className="flex gap-4 flex-col justify-center rounded-md mt-4 msm:w-[20rem] w-[28rem] md:w-[40rem]">
                        <span className="flex gap-4 md:flex-row flex-col justify-between">
                            <BasicInfoInput />
                            <OrganizationInfoInput userType={userType} />
                        </span>
                        <span className="grid place-items-center">
                            <Button type="submit">
                                Signup
                            </Button>
                        </span>
                    </form>
                </CardContent>
            </Card>
        </span>
    )
}

function BasicInfoInput() {
    return (
        <span className="space-y-4 w-full">
            <h1 className="text-2xl font-semibold">Basic Information</h1>
            <Input name="firstName" placeholder="First Name" required />
            <Input name="lastName" placeholder="Last Name" required />
            <Input name="email" placeholder="Email" type="email" required />
            <Input name="contact" placeholder="Contact" required />
            <Input name="password" placeholder="Password" type="password" required />
            <Input name="confirmPassword" placeholder="Confirm Password" type="password" required />
        </span>
    );
}

function OrganizationInfoInput({ userType }: { userType: string }) {
    const [orgExist, setOrgExist] = useState(false);
    const [indInvestor, setIndInvestor] = useState(false);

    return (
        <span className="space-y-4 w-full">
            <h1 className="text-2xl font-semibold">Organization Information</h1>
            {
                userType === "manager" &&
                <CheckBoxWithLabel
                    label="Organization Already Exits?"
                    id="orgExist"
                    onClick={() => setOrgExist(!orgExist)}
                />
            }
            {
                userType === "investor" &&
                <CheckBoxWithLabel
                    label="Signup As an Individual Investor?"
                    id="individualInvestor"
                    onClick={() => setIndInvestor(!indInvestor)}
                />
            }
            {
                !indInvestor &&
                <Input name="organization" placeholder="Organization" required />
            }
            {
                (userType === "manager" || userType === "investor") && !orgExist && !indInvestor &&
                <>
                    <Input name="orgContact" placeholder="Contact" required />
                    <Input name="orgEmail" placeholder="Email" type="email" required />
                    <Input name="orgLocation" placeholder="Location" />
                </>
            }
        </span>
    )
}