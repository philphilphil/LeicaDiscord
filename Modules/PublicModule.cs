using System.IO;
using System.Threading.Tasks;
using Discord;
using Discord.Commands;

namespace KenR_LeicaBot.Modules
{
    // Modules must be public and inherit from an IModuleBase
    public class PublicModule : ModuleBase<SocketCommandContext>
    {
        // Dependency Injection will fill this value in for us
        //public PictureService PictureService { get; set; }

        [Command("50mm")]
        [Alias("hi")]
        public Task PingAsync()
            => ReplyAsync("LEICA 50mm f/2 SUMMICRON - M (1979 - today) - There is no better 50mm lens on Earth, or anywhere.");


        // Get info on a user, or the user who invoked the command if one is not specified
        [Command("userinfo")]
        public async Task UserInfoAsync(IUser user = null)
        {
            user = user ?? Context.User;

            await ReplyAsync(user.ToString());
        }

 
    }
}
