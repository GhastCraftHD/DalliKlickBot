package de.leghast.dalliklick.holder;

import de.leghast.dalliklick.game.Difficulty;
import net.dv8tion.jda.api.interactions.commands.Command;
import net.dv8tion.jda.api.interactions.commands.DefaultMemberPermissions;
import net.dv8tion.jda.api.interactions.commands.OptionType;
import net.dv8tion.jda.api.interactions.commands.build.CommandData;
import net.dv8tion.jda.api.interactions.commands.build.Commands;
import net.dv8tion.jda.api.interactions.commands.build.OptionData;

import java.util.Arrays;
import java.util.List;

public class BotCommands {

    private static final List<Command.Choice> uploadChoices = Arrays.stream(Difficulty.values())
            .map(diff -> new Command.Choice(diff.prettyName(), diff.prettyName()))
            .toList();

    public static final List<CommandData> COMMAND_DATA = List.of(
            Commands.slash("upload", "Upload a new DalliKlick to the database")
                    .addOption(OptionType.ATTACHMENT, "image", "The image of the Dalli Klick subject")
                    .addOption(OptionType.STRING, "subject", "The right answer to the Dalli Klick")
                    .addOptions(
                            new OptionData(OptionType.STRING, "difficulty", "The difficulty of the Dalli Klick")
                                    .addChoices(uploadChoices)
                    )
                    .setDefaultPermissions(DefaultMemberPermissions.DISABLED),
            Commands.message("Dalli Klick bearbeiten"),
            Commands.slash("dalliklick", "Eine Erklärung, was Dalli Klick ist und wie man mitspielen kann")
    );

}
