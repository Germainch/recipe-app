import type {Ingredient} from "$lib/models/ingredient";

export type Recipe = {
    id: number;
    name: string;
    steps: string;
    created_by?: number;
}


export const DefaultRecipe: Recipe = {
    id: 0,
    name: "CARROT CAKE",
    steps:
        "Preheat oven to 350 degrees F (175 degrees C). Grease and flour a 9x13 inch pan.\n " +
        "In a large bowl, beat together eggs, oil, white sugar and 2 teaspoons vanilla. Mix in flour, baking soda, baking powder, salt and cinnamon. Stir in carrots. Fold in pecans. Pour into prepared pan.\n" +
        "Bake in the preheated oven for 40 to 50 minutes, or until a toothpick inserted into the center of the cake comes out clean. Let cool in pan for 10 minutes, then turn out onto a wire rack and cool completely.\n" +
        "To Make Frosting: In a medium bowl, combine butter, cream cheese, confectioners' sugar and 1 teaspoon vanilla. Beat until the mixture is smooth and creamy. Stir in chopped pecans. Frost the cooled cake.",
}