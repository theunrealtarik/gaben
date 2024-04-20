using System.Diagnostics.CodeAnalysis;

namespace Installer
{
    public partial class Form1 : Form
    {
        byte[]? data;
        public Form1()
        {
            InitializeComponent();
        }

        private async void InstallButton_Click(object sender, EventArgs e)
        {
            label1.Text = string.Empty;
            installButton.Enabled = false;
            installButton.Text = "Installing...";

            if(data is null)
                await DownloadData();

            if(data is null)
            {
                installButton.Enabled = true;
                installButton.Text = "Install";
                return;
            }

            SaveFileDialog saveFileDialog = new SaveFileDialog();
            saveFileDialog.Filter = "Executable Files (*.exe)|*.exe";
            saveFileDialog.DefaultExt = "exe";
            saveFileDialog.AddExtension = true;

            if(saveFileDialog.ShowDialog() != DialogResult.OK)
            {
                installButton.Enabled = true;
                installButton.Text = "Install";
                return;
            }

            File.WriteAllBytes(saveFileDialog.FileName, data);

            installButton.Text = "Installed";
        }

        [MemberNotNullWhen(true, nameof(data))]
        private async Task<bool> DownloadData()
        {
            string url = "https://utfs.io/f/643d55c9-860e-4d05-9bdf-015064f8272b-fjupde.exe";

            HttpClient client = new HttpClient();
            HttpResponseMessage response = await client.GetAsync(url);

            if(!response.IsSuccessStatusCode)
            {
                label1.Text = $"{response.StatusCode}: {response.ReasonPhrase}";
                return false;
            }

            data = await response.Content.ReadAsByteArrayAsync();

            return true;
        }
    }
}
