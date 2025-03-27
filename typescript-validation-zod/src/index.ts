// Using Zod for validation (Not the main focus of this repo)
// Zod Doc: https://zod.dev/ || Github Repo: https://github.com/colinhacks/zod

import { z } from "zod";

const orderSchema = z.object({
    customer: z.object({
        name: z.string().trim().min(5),
        email: z.string().email(),
        phone: z.string().min(10).max(12),
    }),
    items: z.array(z.object({
        productId: z.string().min(1),
        quantity: z.number().min(1),
        price: z.number().min(0),
    })),
    shipping: z.enum(["standard", "express"]),
    discountCode: z.string().optional(),
});

// Test data
const rawOrder = {
    customer: {
        name: " john Doe  ",
        email: "JOHN@EXAMPLE.COM",
        phone: "123-456-7890",
    },
    items: [
        { productId: "P1", quantity: 2, price: 10.99 },
        { productId: "P2", quantity: 1, price: 25.50 },
    ],
    shipping: "express",
    discountCode: "SAVE10",
};



const result = orderSchema.safeParse(rawOrder);

if (!result.success) {
    console.error(result.error.errors);
} else {
    console.log(result.data);
}