//! We are so sorry.
//! This code is a mess, it's not our fault, we swear.
//! Blame Microsoft, they made us do it.
//? - checksum, sargon64 - 2023

use crate::structs::Instance;
use forge_lib::structs::v1::ManifestBuilder;
use semver::{Version, VersionReq};
use uuid::Uuid;

pub struct ForgeGenerator {
    pub name: String,
    pub path: String,
    pub instance: Instance,

    // generator fields
    pub section_uuid: Uuid,
    pub sln_uuid: Uuid,
    pub csproj_uuid: Uuid,
}

impl ForgeGenerator {
    pub fn new(name: String, path: String, instance: Instance) -> ForgeGenerator {
        ForgeGenerator {
            name,
            path,
            instance,
            section_uuid: Uuid::new_v4(),
            sln_uuid: Uuid::new_v4(),
            csproj_uuid: Uuid::new_v4(),
        }
    }

    pub fn generate(self) {
        // root files
        let readme = self.make_readme();
        let gitignore = self.make_gitignore();
        let solution = self.make_sln();

        // project files
        let csproj = self.make_csproj();
        let csproj_user = self.make_csproj_user();
        let dir_build_props = self.make_dir_build_props();
        let assembly_info = self.make_assembly_info();

        // plugin files
        let plugin = self.make_plugin();
        let plugin_conf = self.make_plugin_conf();
        let plugin_controller = self.make_plugin_controller();
        let ipa_manifest = self.make_ipa_manifest();

        let _ = std::fs::create_dir_all(&self.path);

        std::fs::write(format!("{}/README.md", self.path), readme).unwrap();
        std::fs::write(format!("{}/.gitignore", self.path), gitignore).unwrap();
        std::fs::write(format!("{}/{}.sln", self.path, self.name), solution).unwrap();

        std::fs::create_dir(format!("{}/{}", self.path, self.name)).unwrap();
        std::fs::create_dir(format!("{}/{}/Properties", self.path, self.name)).unwrap();
        std::fs::create_dir(format!("{}/{}/Configuration", self.path, self.name)).unwrap();

        std::fs::write(format!("{}/{}/Plugin.cs", self.path, self.name), plugin).unwrap();
        std::fs::write(
            format!("{}/{}/manifest.json", self.path, self.name),
            ipa_manifest,
        )
        .unwrap();

        std::fs::write(
            format!("{}/{}/beatforge.manifest", self.path, self.name),
            self.make_bf_manifest(),
        )
        .unwrap();

        std::fs::write(
            format!("{}/{}/{}.csproj.user", self.path, self.name, self.name),
            csproj_user,
        )
        .unwrap();

        std::fs::write(
            format!("{}/{}/{}Controller.cs", self.path, self.name, self.name),
            plugin_controller,
        )
        .unwrap();

        std::fs::write(
            format!("{}/{}/Configuration/PluginConfig.cs", self.path, self.name),
            plugin_conf,
        )
        .unwrap();

        std::fs::write(
            format!("{}/{}/{}.csproj", self.path, self.name, self.name),
            csproj,
        )
        .unwrap();

        std::fs::write(
            format!("{}/{}/Properties/AssemblyInfo.cs", self.path, self.name),
            assembly_info,
        )
        .unwrap();

        std::fs::write(
            format!("{}/{}/Directory.Build.props", self.path, self.name),
            dir_build_props,
        )
        .unwrap();
    }

    fn make_sln(&self) -> String {
        let name = self.name.clone();
        let section_uuid = self.section_uuid.to_string().to_uppercase();
        let sln_uuid = self.sln_uuid.to_string().to_uppercase();

        format!("Microsoft Visual Studio Solution File, Format Version 12.00
# Visual Studio Version 17
VisualStudioVersion = 17.6.33801.468
MinimumVisualStudioVersion = 10.0.40219.1
Project(\"{{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}}\") = \"{name}\", \"{name}\\{name}.csproj\", \"{section_uuid}\"
EndProject
Global
    GlobalSection(SolutionConfigurationPlatforms) = preSolution
        Debug|Any CPU = Debug|Any CPU
        Release|Any CPU = Release|Any CPU
    EndGlobalSection
    GlobalSection(ProjectConfigurationPlatforms) = postSolution
        {{{section_uuid}}}.Debug|Any CPU.ActiveCfg = Debug|Any CPU
        {{{section_uuid}}}.Debug|Any CPU.Build.0 = Debug|Any CPU
        {{{section_uuid}}}.Release|Any CPU.ActiveCfg = Release|Any CPU
        {{{section_uuid}}}.Release|Any CPU.Build.0 = Release|Any CPU
    EndGlobalSection
    GlobalSection(SolutionProperties) = preSolution
        HideSolutionNode = FALSE
    EndGlobalSection
    GlobalSection(ExtensibilityGlobals) = postSolution
        SolutionGuid = {{{sln_uuid}}}
    EndGlobalSection
EndGlobal")
    }

    fn make_csproj(&self) -> String {
        let name = &self.name;
        let project_guid = self.section_uuid.to_string().to_uppercase();

        format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>
<Project ToolsVersion=\"15.0\" DefaultTargets=\"Build\" xmlns=\"http://schemas.microsoft.com/developer/msbuild/2003\">
    <PropertyGroup>
    <Configuration Condition=\" '$(Configuration)' == '' \">Debug</Configuration>
    <Platform Condition=\" '$(Platform)' == '' \">AnyCPU</Platform>
    <ProductVersion>8.0.30703</ProductVersion>
    <SchemaVersion>2.0</SchemaVersion>
    <ProjectGuid>{{{project_guid}}}</ProjectGuid>
    <OutputType>Library</OutputType>
    <AppDesignerFolder>Properties</AppDesignerFolder>
    <RootNamespace>{name}</RootNamespace>
    <AssemblyName>{name}</AssemblyName>
    <TargetFrameworkVersion>v4.8</TargetFrameworkVersion>
    <FileAlignment>512</FileAlignment>
    <DebugSymbols>true</DebugSymbols>
    <DebugType>portable</DebugType>
    <LocalRefsDir Condition=\"Exists('..\\Refs')\">..\\Refs</LocalRefsDir>
    <GameDirectory>$(LocalRefsDir)</GameDirectory>
    <AppOutputBase>$(MSBuildProjectDirectory)\\</AppOutputBase>
    <!--<PathMap>$(AppOutputBase)=X:\\$(AssemblyName)\\</PathMap>-->
    <ErrorReport>prompt</ErrorReport>
    <WarningLevel>4</WarningLevel>
    </PropertyGroup>
    <PropertyGroup Condition=\" '$(Configuration)' == 'Debug' \">
    <!-- This is required to upload a plugin on beatforge. -->
    <Deterministic>true</Deterministic>
    <Optimize>false</Optimize>
    <OutputPath>bin\\Debug\\</OutputPath>
    <DefineConstants>DEBUG;TRACE</DefineConstants>
    </PropertyGroup>
    <PropertyGroup Condition=\" '$(Configuration)' == 'Release' \">
    <!-- This is required to upload a plugin on beatforge. -->
    <Deterministic>true</Deterministic>
    <Optimize>true</Optimize>
    <OutputPath>bin\\Release\\</OutputPath>
    <ErrorReport>prompt</ErrorReport>
    <WarningLevel>4</WarningLevel>
    </PropertyGroup>
    <PropertyGroup Condition=\"$(DefineConstants.Contains('CIBuild')) OR '$(NCrunch)' == '1'\">
    <DisableCopyToPlugins>True</DisableCopyToPlugins>
    </PropertyGroup>
    <PropertyGroup Condition=\"'$(NCrunch)' == '1'\">
    <DisableCopyToPlugins>True</DisableCopyToPlugins>
    <DisableZipRelease>True</DisableZipRelease>
    </PropertyGroup>
    <ItemGroup>
    <Reference Include=\"System\" />
    <Reference Include=\"System.Core\" />
    <Reference Include=\"System.Xml.Linq\" />
    <Reference Include=\"System.Data.DataSetExtensions\" />
    <Reference Include=\"System.Data\" />
    <Reference Include=\"System.Xml\" />
    <Reference Include=\"Main\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\Main.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"HMLib\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\HMLib.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"HMUI\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\HMUI.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"IPA.Loader\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\IPA.Loader.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"Unity.TextMeshPro\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\Unity.TextMeshPro.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"UnityEngine\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\UnityEngine.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"UnityEngine.CoreModule\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\UnityEngine.CoreModule.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"UnityEngine.UI\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\UnityEngine.UI.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"UnityEngine.UIElementsModule\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\UnityEngine.UIElementsModule.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"UnityEngine.UIModule\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\UnityEngine.UIModule.dll</HintPath>
        <Private>False</Private>
    </Reference>
    <Reference Include=\"UnityEngine.VRModule\">
        <HintPath>$(GameDirectory)\\Beat Saber_Data\\Managed\\UnityEngine.VRModule.dll</HintPath>
        <Private>False</Private>
    </Reference>
    </ItemGroup>
    <ItemGroup>
    <Compile Include=\"Plugin.cs\" />
    <Compile Include=\"Configuration\\PluginConfig.cs\" />
    <Compile Include=\"{name}Controller.cs\" />
    <Compile Include=\"Properties\\AssemblyInfo.cs\" />
    </ItemGroup>
    <ItemGroup>
    <EmbeddedResource Include=\"manifest.json\" />
    </ItemGroup>
    <ItemGroup>
    <EmbeddedResource Include=\"beatforge.manifest\" />
    </ItemGroup>
    <ItemGroup>
    <None Include=\"Directory.Build.props\" Condition=\"Exists('Directory.Build.props')\" />
    <None Include=\"{name}.csproj.user\" Condition=\"Exists('{name}.csproj.user')\" />
    </ItemGroup>
    <ItemGroup>
    <PackageReference Include=\"BeatSaberModdingTools.Tasks\">
        <Version>2.0.0-beta1</Version>
        <IncludeAssets>runtime; build; native; contentfiles; analyzers; buildtransitive</IncludeAssets>
        <PrivateAssets>all</PrivateAssets>
    </PackageReference>
    </ItemGroup>
    <Import Project=\"$(MSBuildToolsPath)\\Microsoft.CSharp.targets\" />
</Project>
")
    }

    fn make_csproj_user(&self) -> String {
        let binding = self.instance.path.clone();
        let ipath = binding.to_str().unwrap();
        format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<Project>
    <PropertyGroup>        
        <!-- Path to your Beat Saber install. -->
        <GameDirectory>{ipath}</GameDirectory>
    </PropertyGroup>
</Project>"
        )
    }

    fn make_plugin_controller(&self) -> String {
        let name = self.name.clone();
        format!("using System;
using System.Collections;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using UnityEngine;

namespace {name}
{{
    /// <summary>
    /// Monobehaviours (scripts) are added to GameObjects.
    /// For a full list of Messages a Monobehaviour can receive from the game, see https://docs.unity3d.com/ScriptReference/MonoBehaviour.html.
    /// </summary>
    public class {name}Controller : MonoBehaviour
    {{
        public static {name}Controller Instance {{ get; private set; }}

        // These methods are automatically called by Unity, you should remove any you aren't using.
        #region Monobehaviour Messages
        /// <summary>
        /// Only ever called once, mainly used to initialize variables.
        /// </summary>
        private void Awake()
        {{
            // For this particular MonoBehaviour, we only want one instance to exist at any time, so store a reference to it in a static property
            //   and destroy any that are created while one already exists.
            if (Instance != null)
            {{
                Plugin.Log?.Warn($\"Instance of {{GetType().Name}} already exists, destroying.\");
                GameObject.DestroyImmediate(this);
                return;
            }}
            GameObject.DontDestroyOnLoad(this); // Don't destroy this object on scene changes
            Instance = this;
            Plugin.Log?.Debug($\"{name}: Awake()\");
        }}
        /// <summary>
        /// Only ever called once on the first frame the script is Enabled. Start is called after any other script's Awake() and before Update().
        /// </summary>
        private void Start()
        {{

        }}

        /// <summary>
        /// Called every frame if the script is enabled.
        /// </summary>
        private void Update()
        {{

        }}

        /// <summary>
        /// Called every frame after every other enabled script's Update().
        /// </summary>
        private void LateUpdate()
        {{

        }}

        /// <summary>
        /// Called when the script becomes enabled and active
        /// </summary>
        private void OnEnable()
        {{

        }}

        /// <summary>
        /// Called when the script becomes disabled or when it is being destroyed.
        /// </summary>
        private void OnDisable()
        {{

        }}

        /// <summary>
        /// Called when the script is being destroyed.
        /// </summary>
        private void OnDestroy()
        {{
            Plugin.Log?.Debug($\"{name}: OnDestroy()\");
            if (Instance == this)
                Instance = null; // This MonoBehaviour is being destroyed, so set the static instance property to null.

        }}
        #endregion
    }}
}}")
    }

    fn make_plugin(&self) -> String {
        let name = self.name.clone();
        format!("using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using System.Threading.Tasks;
using IPA;
using IPA.Config;
using IPA.Config.Stores;
using UnityEngine;
using IPALogger = IPA.Logging.Logger;

namespace {name}
{{
    [Plugin(RuntimeOptions.DynamicInit)]
    public class Plugin
    {{
        // TODO: If using Harmony, uncomment and change YourGitHub to the name of your GitHub account, or use the form \"com.company.project.product\"
        //       You must also add a reference to the Harmony assembly in the Libs folder.
        // public const string HarmonyId = \"com.github.YourGitHub.{name}\";
        // internal static readonly HarmonyLib.Harmony harmony = new HarmonyLib.Harmony(HarmonyId);

        internal static Plugin Instance {{ get; private set; }}
        internal static IPALogger Log {{ get; private set; }}
        internal static {name}Controller PluginController {{ get {{ return {name}Controller.Instance; }} }}

        [Init]
        /// <summary>
        /// Called when the plugin is first loaded by IPA (either when the game starts or when the plugin is enabled if it starts disabled).
        /// [Init] methods that use a Constructor or called before regular methods like InitWithConfig.
        /// Only use [Init] with one Constructor.
        /// </summary>
        public Plugin(IPALogger logger)
        {{
            Instance = this;
            Plugin.Log = logger;
            Plugin.Log?.Debug(\"Logger initialized.\");
        }}

        #region BSIPA Config
        //Uncomment to use BSIPA's config
        /*
        [Init]
        public void InitWithConfig(Config conf)
        {{
            Configuration.PluginConfig.Instance = conf.Generated<Configuration.PluginConfig>();
            Plugin.Log?.Debug(\"Config loaded\");
        }}
        */
        #endregion


        #region Disableable

        /// <summary>
        /// Called when the plugin is enabled (including when the game starts if the plugin is enabled).
        /// </summary>
        [OnEnable]
        public void OnEnable()
        {{
            new GameObject(\"{name}Controller\").AddComponent<{name}Controller>();
            //ApplyHarmonyPatches();
        }}

        /// <summary>
        /// Called when the plugin is disabled and on Beat Saber quit. It is important to clean up any Harmony patches, GameObjects, and Monobehaviours here.
        /// The game should be left in a state as if the plugin was never started.
        /// Methods marked [OnDisable] must return void or Task.
        /// </summary>
        [OnDisable]
        public void OnDisable()
        {{
            if (PluginController != null)
                GameObject.Destroy(PluginController);
            //RemoveHarmonyPatches();
        }}

        /*
        /// <summary>
        /// Called when the plugin is disabled and on Beat Saber quit.
        /// Return Task for when the plugin needs to do some long-running, asynchronous work to disable.
        /// [OnDisable] methods that return Task are called after all [OnDisable] methods that return void.
        /// </summary>
        [OnDisable]
        public async Task OnDisableAsync()
        {{
            await LongRunningUnloadTask().ConfigureAwait(false);
        }}
        */
        #endregion

        // Uncomment the methods in this section if using Harmony
        #region Harmony
        /*
        /// <summary>
        /// Attempts to apply all the Harmony patches in this assembly.
        /// </summary>
        internal static void ApplyHarmonyPatches()
        {{
            try
            {{
                Plugin.Log?.Debug(\"Applying Harmony patches.\");
                harmony.PatchAll(Assembly.GetExecutingAssembly());
            }}
            catch (Exception ex)
            {{
                Plugin.Log?.Error(\"Error applying Harmony patches: \" + ex.Message);
                Plugin.Log?.Debug(ex);
            }}
        }}

        /// <summary>
        /// Attempts to remove all the Harmony patches that used our HarmonyId.
        /// </summary>
        internal static void RemoveHarmonyPatches()
        {{
            try
            {{
                // Removes all patches with this HarmonyId
                harmony.UnpatchAll(HarmonyId);
            }}
            catch (Exception ex)
            {{
                Plugin.Log?.Error(\"Error removing Harmony patches: \" + ex.Message);
                Plugin.Log?.Debug(ex);
            }}
        }}
        */
        #endregion
    }}
}}")
    }

    fn make_dir_build_props(&self) -> String {
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>
        <!-- This file contains project properties used by the build. -->
        <Project>
            <PropertyGroup>
                <ImportBSMTTargets>True</ImportBSMTTargets>
                <BSMTProjectType>BSIPA</BSMTProjectType>
            </PropertyGroup>
        </Project>"
            .to_string()
    }
    fn make_assembly_info(&self) -> String {
        let name = self.name.clone();

        format!(
            "using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

// General Information about an assembly is controlled through the following 
// set of attributes. Change these attribute values to modify the information
// associated with an assembly.
[assembly: AssemblyTitle(\"{name}\")]
[assembly: AssemblyDescription(\"\")]
[assembly: AssemblyCompany(\"\")]
[assembly: AssemblyProduct(\"{name}\")]
[assembly: AssemblyCopyright(\"Copyright ©  2023\")]
[assembly: AssemblyTrademark(\"\")]
[assembly: AssemblyCulture(\"\")]

// Setting ComVisible to false makes the types in this assembly not visible 
// to COM components.  If you need to access a type in this assembly from 
// COM, set the ComVisible attribute to true on that type.
[assembly: ComVisible(false)]

// The following GUID is for the ID of the typelib if this project is exposed to COM
[assembly: Guid(\"0c253322-35ed-4891-8cd2-233532a7e71d\")]

// Version information for an assembly consists of the following four values:
//
//      Major Version
//      Minor Version 
//      Build Number
//      Revision
//
// You can specify all the values or you can default the Build and Revision Numbers 
// by using the '*' as shown below:
// [assembly: AssemblyVersion(\"1.0.*\")]
[assembly: AssemblyVersion(\"0.0.1\")]
[assembly: AssemblyFileVersion(\"0.0.1\")]"
        )
    }

    fn make_plugin_conf(&self) -> String {
        let name = self.name.clone();
        format!("/*
using System.Runtime.CompilerServices;
using IPA.Config.Stores;

[assembly: InternalsVisibleTo(GeneratedStore.AssemblyVisibilityTarget)]
namespace {name}.Configuration
{{
    internal class PluginConfig
    {{
        public static PluginConfig Instance {{ get; set; }}
        public virtual int IntValue {{ get; set; }} = 42; // Must be \'virtual\' if you want BSIPA to detect a value change and save the config automatically.

        /// <summary>
        /// This is called whenever BSIPA reads the config from disk (including when file changes are detected).
        /// </summary>
        public virtual void OnReload()
        {{
            // Do stuff after config is read from disk.
        }}

        /// <summary>
        /// Call this to force BSIPA to update the config file. This is also called by BSIPA if it detects the file was modified.
        /// </summary>
        public virtual void Changed()
        {{
            // Do stuff when the config is changed.
        }}

        /// <summary>
        /// Call this to have BSIPA copy the values from <paramref name=\"other\"/> into this config.
        /// </summary>
        public virtual void CopyFrom(PluginConfig other)
        {{
            // This instance's members populated from other
        }}
    }}
}}
*/
")
    }

    fn make_ipa_manifest(&self) -> String {
        let name = self.name.clone();

        format!("{{
    \"$schema\": \"https://raw.githubusercontent.com/bsmg/BSIPA-MetadataFileSchema/master/Schema.json\",
    \"id\": \"{name}\",
    \"name\": \"{name}\",
    \"author\": \"YourNameHere\",
    \"version\": \"0.0.1\",
    \"description\": \"\",
    \"gameVersion\": \"1.29.1\",
    \"dependsOn\": {{
        \"BSIPA\": \"^4.2.0\"
    }}
}}
")
    }

    fn make_bf_manifest(&self) -> String {
        let name = self.name.clone();
        let manifest = ManifestBuilder::new_mod(
            name,
            Version::new(0, 0, 1),
            VersionReq::STAR,
            self.path.clone().into(),
        )
        .build();

        serde_json::to_string_pretty(&manifest).unwrap()
    }

    fn make_gitignore(&self) -> String {
        "bin/
        obj/
        *.user
        *.suo
        *.userprefs
        *.pidb
        *.sln.docstates"
            .to_string()
    }

    fn make_readme(&self) -> String {
        let name = self.name.clone();

        format!(
            "# {name}

This is an autogenerated project using [BeatForge](https://beatforge.net)"
        )
    }
}
